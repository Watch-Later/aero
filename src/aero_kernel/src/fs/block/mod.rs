/*
 * Copyright (C) 2021-2022 The Aero Project Developers.
 *
 * This file is part of The Aero Project.
 *
 * Aero is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Aero is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Aero. If not, see <https://www.gnu.org/licenses/>.
 */

mod gpt;

use alloc::boxed::Box;
use gpt::Gpt;

use core::mem::MaybeUninit;

use alloc::collections::BTreeMap;
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;

use crate::fs::devfs::install_device;
use crate::fs::{Path, Result, MOUNT_MANAGER};

use crate::fs::ext2::Ext2;
use crate::mem::paging::{align_down, align_up};
use crate::utils::sync::Mutex;

use super::devfs::{alloc_device_marker, Device};
use super::inode::INodeInterface;

pub trait BlockDeviceInterface: Send + Sync {
    fn block_size(&self) -> usize;

    fn read_block(&self, sector: usize, dest: &mut [MaybeUninit<u8>]) -> Option<usize>;
    fn write_block(&self, sector: usize, buf: &[u8]) -> Option<usize>;

    fn read(&self, offset: usize, dest: &mut [MaybeUninit<u8>]) -> Option<usize> {
        let aligned_offset = align_down(offset as u64, self.block_size() as u64) as usize;
        let sector = aligned_offset / self.block_size();

        let aligned_size = align_up(dest.len() as u64, self.block_size() as u64) as usize;
        let mut buffer = Box::<[u8]>::new_uninit_slice(aligned_size);

        self.read_block(sector, MaybeUninit::slice_as_bytes_mut(&mut buffer))?;
        // SAFETY: We have initialized the buffer above.
        let buffer = unsafe { buffer.assume_init() };

        let offset = offset - aligned_offset;
        MaybeUninit::write_slice(dest, &buffer[offset..(offset + dest.len())]);

        Some(dest.len())
    }
}

static BLOCK_DEVS: Mutex<BTreeMap<usize, Arc<BlockDevice>>> = Mutex::new(BTreeMap::new());

/// Installs the provided block `device` into the filesyetm.
pub fn install_block_device(dev: Arc<BlockDevice>) -> Result<()> {
    let mut devs = BLOCK_DEVS.lock();
    install_device(dev.clone())?;

    log::debug!("block: installed block device {}", dev.name());
    devs.insert(dev.id, dev);

    Ok(())
}

pub struct BlockDevice {
    id: usize,
    name: String,
    dev: Arc<dyn BlockDeviceInterface>,
    self_ref: Weak<BlockDevice>,
}

impl BlockDevice {
    pub fn new(name: String, imp: Arc<dyn BlockDeviceInterface>) -> Arc<BlockDevice> {
        Arc::new_cyclic(|me| BlockDevice {
            id: alloc_device_marker(),
            name,
            dev: imp,
            self_ref: me.clone(),
        })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn device(&self) -> Arc<dyn BlockDeviceInterface> {
        self.dev.clone()
    }
}

impl INodeInterface for BlockDevice {}

impl Device for BlockDevice {
    fn device_marker(&self) -> usize {
        self.id
    }

    fn device_name(&self) -> String {
        self.name()
    }

    fn inode(&self) -> Arc<dyn INodeInterface> {
        self.self_ref.upgrade().unwrap().clone()
    }
}

struct PartitionBlockDevice {
    offset: usize, // offset in sectors
    size: usize,   // capacity in sectors
    device: Arc<dyn BlockDeviceInterface>,
}

impl PartitionBlockDevice {
    fn new(offset: usize, size: usize, device: Arc<dyn BlockDeviceInterface>) -> Self {
        Self {
            offset,
            size,
            device,
        }
    }
}

impl BlockDeviceInterface for PartitionBlockDevice {
    fn read_block(&self, sector: usize, dest: &mut [MaybeUninit<u8>]) -> Option<usize> {
        if sector >= self.size {
            return None;
        }

        self.device.read_block(self.offset + sector, dest)
    }

    fn block_size(&self) -> usize {
        self.device.block_size()
    }

    fn write_block(&self, sector: usize, buf: &[u8]) -> Option<usize> {
        if sector >= self.size {
            return None;
        }

        self.device.write_block(self.offset + sector, buf)
    }
}

pub fn launch() -> Result<()> {
    let mut blocks_copy = Vec::<Arc<BlockDevice>>::new();

    for (_, device) in BLOCK_DEVS.lock().iter() {
        blocks_copy.push(device.clone());
    }

    for block in blocks_copy {
        if let Some(gpt) = Gpt::new(block.clone()) {
            log::info!("block: found GPT on {}!", block.name());

            for (i, entry) in gpt
                .entries()
                .iter()
                .enumerate()
                .filter(|(_, e)| e.is_used())
            {
                let start = entry.start_lba() as usize;
                let size = entry.size() as usize;

                log::info!(
                    "gpt: found partition (name=`{}`, start={:#x}, size{:#x})!",
                    entry.partition_name(),
                    start,
                    size
                );

                let name = alloc::format!("{}p{}", block.name(), i);
                let partition_device = PartitionBlockDevice::new(start, size, block.device());
                let device = BlockDevice::new(name, Arc::new(partition_device));

                install_block_device(device.clone())?;

                // Check what filesystem is on this partition and mount it.
                if let Some(ext2) = Ext2::new(device.clone()) {
                    log::info!("gpt: found ext2 filesystem on {}!", device.name());
                    MOUNT_MANAGER.mount(super::lookup_path(Path::new("/mnt"))?, ext2)?;
                }
            }
        }
    }

    Ok(())
}
