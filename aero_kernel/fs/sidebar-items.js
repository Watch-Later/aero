initSidebarItems({"enum":[["FileSystemError",""],["LookupMode",""]],"fn":[["init",""],["lookup_path",""],["lookup_path_with",""],["lookup_path_with_mode",""],["root_dir",""]],"mod":[["block",""],["cache","General implementation for file system caching. Stuff like inode needs to be cached to improve performance and in this case looking up inode data from an IO device such as a disk is very slow, so storing previously accessed inode data in memory makes file system access much quicker."],["devfs","The `/dev` directory contains the special device files for all the devices."],["file_table",""],["initramfs",""],["inode",""],["pipe",""],["ramfs","Implementation of in-memory filesystem. This is used for temporary filesystems (e.g. dev, temp) and since Aero currently does not have support for actual disk filesystems (e.g. ex2 and FAT32), ram-fs is used as the root filesystem."]],"static":[["ROOT_DIR",""],["ROOT_FS",""]],"struct":[["MOUNT_MANAGER",""],["MountManager",""],["MountPoint",""],["Path","A slice of a path (akin to [str])."]],"trait":[["FileSystem",""]],"type":[["MountKey",""],["Result",""]]});