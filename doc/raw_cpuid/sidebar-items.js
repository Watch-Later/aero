initSidebarItems({"constant":[["CACHE_INFO_TABLE","This table is taken from Intel manual (Section CPUID instruction)."]],"enum":[["CacheInfoType","What type of cache are we dealing with?"],["CacheType",""],["DatType","Deterministic Address Translation cache type (EDX bits 04 – 00)"],["Hypervisor","Identifies the different Hypervisor products."],["L2Associativity",""],["SgxSectionInfo","Intel SGX EPC Enumeration Leaf, sub-leaves (EAX = 12H, ECX = 2 or higher)"],["TopologyType","What type of core we have at this level in the topology (real CPU or hyper-threaded)."]],"macro":[["cpuid","Macro which queries cpuid directly."]],"mod":[["native_cpuid","Uses Rust’s `cpuid` function from the `arch` module."]],"struct":[["CacheInfo","Describes any kind of cache (TLB, Data and Instruction caches plus prefetchers)."],["CacheInfoIter","Used to iterate over cache information contained in cpuid instruction."],["CacheParameter",""],["CacheParametersIter",""],["CpuId","Main type used to query for information about the CPU we’re running on."],["CpuIdResult","Low-level data-structure to store result of cpuid instruction."],["DatInfo","Deterministic Address Translation Structure"],["DatIter","Deterministic Address Translation Structure Iterator"],["DirectCacheAccessInfo",""],["EpcSection","EBX:EAX and EDX:ECX provide information on the Enclave Page Cache (EPC) section"],["ExtendedFeatures",""],["ExtendedFunctionInfo",""],["ExtendedState",""],["ExtendedStateInfo",""],["ExtendedStateIter",""],["ExtendedTopologyIter","Iterates over the system topology in order to retrieve more system information at each level of the topology."],["ExtendedTopologyLevel","Gives detailed information about the current level in the topology (how many cores, what type etc.)."],["FeatureInfo",""],["HypervisorInfo","Information about Hypervisor (https://lwn.net/Articles/301888/)"],["L2CatInfo","L2 Cache Allocation Technology Enumeration Sub-leaf (EAX = 10H, ECX = ResID = 2)."],["L3CatInfo","L3 Cache Allocation Technology Enumeration Sub-leaf (EAX = 10H, ECX = ResID = 1)."],["L3MonitoringInfo",""],["MemBwAllocationInfo","Memory Bandwidth Allocation Enumeration Sub-leaf (EAX = 10H, ECX = ResID = 3)."],["MemoryEncryptionInfo",""],["MonitorMwaitInfo",""],["PerformanceMonitoringInfo",""],["ProcessorFrequencyInfo","Processor Frequency Information"],["ProcessorSerial",""],["ProcessorTraceInfo",""],["RdtAllocationInfo",""],["RdtMonitoringInfo",""],["SgxInfo","Intel SGX Capability Enumeration Leaf, sub-leaf 0 (EAX = 12H, ECX = 0 and ECX = 1)"],["SgxSectionIter","Iterator over the SGX sub-leafs (ECX >= 2)."],["SoCVendorAttributesIter",""],["SoCVendorBrand",""],["SoCVendorInfo",""],["ThermalPowerInfo",""],["TscInfo","Time Stamp Counter and Nominal Core Crystal Clock Information Leaf."],["VendorInfo",""]]});