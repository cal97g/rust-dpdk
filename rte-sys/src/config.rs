/* automatically generated by rte-sys v18.11.0, DON'T EDIT IT */

/// SPDX-License-Identifier: BSD-3-Clause
/// Copyright(c) 2010-2014 Intel Corporation
/// SPDX-License-Identifier: BSD-3-Clause
/// Copyright(c) 2010-2016 Intel Corporation
/// SPDX-License-Identifier: BSD-3-Clause
/// Copyright(c) 2010-2017 Intel Corporation
/// RTE_EXEC_ENV values are the directories in mk/exec-env/
pub const CONFIG_RTE_EXEC_ENV: &str = "linuxapp";
/// RTE_ARCH values are architecture we compile for. directories in mk/arch/
pub const CONFIG_RTE_ARCH: &str = "x86_64";
/// machine can define specific variables or action for a specific board
/// RTE_MACHINE values are architecture we compile for. directories in mk/machine/
pub const CONFIG_RTE_MACHINE: &str = "native";
/// The compiler we use.
/// RTE_TOOLCHAIN values are architecture we compile for. directories in mk/toolchain/
pub const CONFIG_RTE_TOOLCHAIN: &str = "gcc";
/// Use intrinsics or assembly code for key routines
pub const CONFIG_RTE_FORCE_INTRINSICS: bool = false;
/// Machine forces strict alignment constraints.
pub const CONFIG_RTE_ARCH_STRICT_ALIGN: bool = false;
/// Compile to share library
pub const CONFIG_RTE_BUILD_SHARED_LIB: bool = false;
/// Use newest code breaking previous ABI
pub const CONFIG_RTE_NEXT_ABI: bool = true;
/// Major ABI to overwrite library specific LIBABIVER
pub const CONFIG_RTE_MAJOR_ABI: () = ();
/// Machine's cache line size
pub const CONFIG_RTE_CACHE_LINE_SIZE: u32 = 64;
/// Memory model
pub const CONFIG_RTE_USE_C11_MEM_MODEL: bool = false;
/// Compile Environment Abstraction Layer
pub const CONFIG_RTE_LIBRTE_EAL: bool = true;
pub const CONFIG_RTE_MAX_LCORE: u32 = 128;
pub const CONFIG_RTE_MAX_NUMA_NODES: u32 = 8;
pub const CONFIG_RTE_MAX_HEAPS: u32 = 32;
pub const CONFIG_RTE_MAX_MEMSEG_LISTS: u32 = 64;
/// each memseg list will be limited to either RTE_MAX_MEMSEG_PER_LIST pages
/// or RTE_MAX_MEM_MB_PER_LIST megabytes worth of memory, whichever is smaller
pub const CONFIG_RTE_MAX_MEMSEG_PER_LIST: u32 = 8192;
pub const CONFIG_RTE_MAX_MEM_MB_PER_LIST: u32 = 32768;
/// a "type" is a combination of page size and NUMA node. total number of memseg
/// lists per type will be limited to either RTE_MAX_MEMSEG_PER_TYPE pages (split
/// over multiple lists of RTE_MAX_MEMSEG_PER_LIST pages), or
/// RTE_MAX_MEM_MB_PER_TYPE megabytes of memory (split over multiple lists of
/// RTE_MAX_MEM_MB_PER_LIST), whichever is smaller
pub const CONFIG_RTE_MAX_MEMSEG_PER_TYPE: u32 = 32768;
pub const CONFIG_RTE_MAX_MEM_MB_PER_TYPE: u32 = 131072;
/// global maximum usable amount of VA, in megabytes
pub const CONFIG_RTE_MAX_MEM_MB: u32 = 524288;
pub const CONFIG_RTE_MAX_MEMZONE: u32 = 2560;
pub const CONFIG_RTE_MAX_TAILQ: u32 = 32;
pub const CONFIG_RTE_ENABLE_ASSERT: bool = false;
// pub const CONFIG_RTE_LOG_DP_LEVEL: _ = RTE_LOG_INFO;
pub const CONFIG_RTE_LOG_HISTORY: u32 = 256;
pub const CONFIG_RTE_BACKTRACE: bool = true;
pub const CONFIG_RTE_LIBEAL_USE_HPET: bool = false;
pub const CONFIG_RTE_EAL_ALLOW_INV_SOCKET_ID: bool = false;
pub const CONFIG_RTE_EAL_ALWAYS_PANIC_ON_ERROR: bool = false;
pub const CONFIG_RTE_EAL_IGB_UIO: bool = true;
pub const CONFIG_RTE_EAL_VFIO: bool = true;
pub const CONFIG_RTE_MAX_VFIO_GROUPS: u32 = 64;
pub const CONFIG_RTE_MAX_VFIO_CONTAINERS: u32 = 64;
pub const CONFIG_RTE_MALLOC_DEBUG: bool = false;
pub const CONFIG_RTE_EAL_NUMA_AWARE_HUGEPAGES: bool = true;
pub const CONFIG_RTE_USE_LIBBSD: bool = false;
/// Recognize/ignore architecture we compile for. AVX/AVX512 CPU flags for performance/power testing.
/// AVX512 is marked as experimental for now, will enable it after enough
/// field test and possible optimization.
pub const CONFIG_RTE_ENABLE_AVX: bool = true;
pub const CONFIG_RTE_ENABLE_AVX512: bool = false;
/// Default driver path (or "" to disable)
pub const CONFIG_RTE_EAL_PMD_PATH: &str = "";
/// Compile Environment Abstraction Layer to support Vmware TSC map
pub const CONFIG_RTE_LIBRTE_EAL_VMWARE_TSC_MAP_SUPPORT: bool = true;
/// Compile architecture we compile for. PCI library
pub const CONFIG_RTE_LIBRTE_PCI: bool = true;
/// Compile architecture we compile for. argument parser library
pub const CONFIG_RTE_LIBRTE_KVARGS: bool = true;
/// Compile generic ethernet library
pub const CONFIG_RTE_LIBRTE_ETHER: bool = true;
pub const CONFIG_RTE_LIBRTE_ETHDEV_DEBUG: bool = false;
pub const CONFIG_RTE_MAX_ETHPORTS: u32 = 32;
pub const CONFIG_RTE_MAX_QUEUES_PER_PORT: u32 = 1024;
pub const CONFIG_RTE_LIBRTE_IEEE1588: bool = false;
pub const CONFIG_RTE_ETHDEV_QUEUE_STAT_CNTRS: u32 = 16;
pub const CONFIG_RTE_ETHDEV_RXTX_CALLBACKS: bool = true;
pub const CONFIG_RTE_ETHDEV_PROFILE_WITH_VTUNE: bool = false;
/// Turn off Tx preparation stage
/// Warning: rte_eth_tx_prepare() can be safely disabled only if using a
/// driver which do not implement any Tx preparation.
pub const CONFIG_RTE_ETHDEV_TX_PREPARE_NOOP: bool = false;
/// Common libraries, before Bus/PMDs
pub const CONFIG_RTE_LIBRTE_COMMON_DPAAX: bool = true;
/// Compile architecture we compile for. Intel FPGA bus
pub const CONFIG_RTE_LIBRTE_IFPGA_BUS: bool = true;
/// Compile PCI bus driver
pub const CONFIG_RTE_LIBRTE_PCI_BUS: bool = true;
/// Compile architecture we compile for. vdev bus
pub const CONFIG_RTE_LIBRTE_VDEV_BUS: bool = true;
/// Compile ARK PMD
pub const CONFIG_RTE_LIBRTE_ARK_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_ARK_PAD_TX: bool = true;
pub const CONFIG_RTE_LIBRTE_ARK_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_ARK_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_ARK_DEBUG_STATS: bool = false;
pub const CONFIG_RTE_LIBRTE_ARK_DEBUG_TRACE: bool = false;
/// Compile Aquantia Atlantic PMD driver
pub const CONFIG_RTE_LIBRTE_ATLANTIC_PMD: bool = true;
/// Compile AMD PMD
pub const CONFIG_RTE_LIBRTE_AXGBE_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_AXGBE_PMD_DEBUG: bool = false;
/// Compile burst-oriented Broadcom PMD driver
pub const CONFIG_RTE_LIBRTE_BNX2X_PMD: bool = false;
pub const CONFIG_RTE_LIBRTE_BNX2X_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_BNX2X_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_BNX2X_MF_SUPPORT: bool = false;
pub const CONFIG_RTE_LIBRTE_BNX2X_DEBUG_PERIODIC: bool = false;
/// Compile burst-oriented Broadcom BNXT PMD driver
pub const CONFIG_RTE_LIBRTE_BNXT_PMD: bool = true;
/// Compile burst-oriented Chelsio Terminator (CXGBE) PMD
pub const CONFIG_RTE_LIBRTE_CXGBE_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_CXGBE_DEBUG: bool = false;
pub const CONFIG_RTE_LIBRTE_CXGBE_DEBUG_REG: bool = false;
pub const CONFIG_RTE_LIBRTE_CXGBE_DEBUG_MBOX: bool = false;
pub const CONFIG_RTE_LIBRTE_CXGBE_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_CXGBE_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_CXGBE_TPUT: bool = true;
/// NXP DPAA Bus
pub const CONFIG_RTE_LIBRTE_DPAA_BUS: bool = true;
pub const CONFIG_RTE_LIBRTE_DPAA_MEMPOOL: bool = true;
pub const CONFIG_RTE_LIBRTE_DPAA_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_DPAA_HWDEBUG: bool = false;
/// Compile NXP DPAA2 FSL-MC Bus
pub const CONFIG_RTE_LIBRTE_FSLMC_BUS: bool = true;
/// Compile Support Libraries for NXP DPAA2
pub const CONFIG_RTE_LIBRTE_DPAA2_MEMPOOL: bool = true;
pub const CONFIG_RTE_LIBRTE_DPAA2_USE_PHYS_IOVA: bool = true;
/// Compile burst-oriented NXP DPAA2 PMD driver
pub const CONFIG_RTE_LIBRTE_DPAA2_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_DPAA2_DEBUG_DRIVER: bool = false;
/// Compile NXP ENETC PMD Driver
pub const CONFIG_RTE_LIBRTE_ENETC_PMD: bool = true;
/// Compile burst-oriented Amazon ENA PMD driver
pub const CONFIG_RTE_LIBRTE_ENA_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_ENA_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_ENA_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_ENA_DEBUG_TX_FREE: bool = false;
pub const CONFIG_RTE_LIBRTE_ENA_COM_DEBUG: bool = false;
/// Compile burst-oriented Cisco ENIC PMD driver
pub const CONFIG_RTE_LIBRTE_ENIC_PMD: bool = true;
/// Compile burst-oriented IGB & EM PMD drivers
pub const CONFIG_RTE_LIBRTE_EM_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_IGB_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_E1000_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_E1000_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_E1000_DEBUG_TX_FREE: bool = false;
pub const CONFIG_RTE_LIBRTE_E1000_PF_DISABLE_STRIP_CRC: bool = false;
/// Compile burst-oriented IXGBE PMD driver
pub const CONFIG_RTE_LIBRTE_IXGBE_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_IXGBE_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_IXGBE_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_IXGBE_DEBUG_TX_FREE: bool = false;
pub const CONFIG_RTE_LIBRTE_IXGBE_PF_DISABLE_STRIP_CRC: bool = false;
pub const CONFIG_RTE_IXGBE_INC_VECTOR: bool = true;
pub const CONFIG_RTE_LIBRTE_IXGBE_BYPASS: bool = false;
/// Compile burst-oriented I40E PMD driver
pub const CONFIG_RTE_LIBRTE_I40E_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_I40E_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_I40E_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_I40E_DEBUG_TX_FREE: bool = false;
pub const CONFIG_RTE_LIBRTE_I40E_RX_ALLOW_BULK_ALLOC: bool = true;
pub const CONFIG_RTE_LIBRTE_I40E_INC_VECTOR: bool = true;
pub const CONFIG_RTE_LIBRTE_I40E_16BYTE_RX_DESC: bool = false;
pub const CONFIG_RTE_LIBRTE_I40E_QUEUE_NUM_PER_PF: u32 = 64;
pub const CONFIG_RTE_LIBRTE_I40E_QUEUE_NUM_PER_VM: u32 = 4;
/// Compile burst-oriented FM10K PMD
pub const CONFIG_RTE_LIBRTE_FM10K_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_FM10K_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_FM10K_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_FM10K_DEBUG_TX_FREE: bool = false;
pub const CONFIG_RTE_LIBRTE_FM10K_RX_OLFLAGS_ENABLE: bool = true;
pub const CONFIG_RTE_LIBRTE_FM10K_INC_VECTOR: bool = true;
/// Compile burst-oriented AVF PMD driver
pub const CONFIG_RTE_LIBRTE_AVF_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_AVF_INC_VECTOR: bool = true;
pub const CONFIG_RTE_LIBRTE_AVF_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_AVF_DEBUG_TX_FREE: bool = false;
pub const CONFIG_RTE_LIBRTE_AVF_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_AVF_16BYTE_RX_DESC: bool = false;
/// Compile burst-oriented Mellanox ConnectX-3 (MLX4) PMD
pub const CONFIG_RTE_LIBRTE_MLX4_PMD: bool = false;
pub const CONFIG_RTE_LIBRTE_MLX4_DEBUG: bool = false;
pub const CONFIG_RTE_LIBRTE_MLX4_DLOPEN_DEPS: bool = false;
/// Compile burst-oriented Mellanox ConnectX-4, ConnectX-5 & Bluefield
/// (MLX5) PMD
pub const CONFIG_RTE_LIBRTE_MLX5_PMD: bool = false;
pub const CONFIG_RTE_LIBRTE_MLX5_DEBUG: bool = false;
pub const CONFIG_RTE_LIBRTE_MLX5_DLOPEN_DEPS: bool = false;
/// Compile burst-oriented Netronome NFP PMD driver
pub const CONFIG_RTE_LIBRTE_NFP_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_NFP_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_NFP_DEBUG_RX: bool = false;
/// QLogic 10G/25G/40G/50G/100G PMD
pub const CONFIG_RTE_LIBRTE_QEDE_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_QEDE_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_QEDE_DEBUG_RX: bool = false;
///Provides abs path/name of architecture we compile for. firmware file.
///Empty string denotes driver will use default firmware
pub const CONFIG_RTE_LIBRTE_QEDE_FW: &str = "";
/// Compile burst-oriented Solarflare libefx-based PMD
pub const CONFIG_RTE_LIBRTE_SFC_EFX_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_SFC_EFX_DEBUG: bool = false;
/// Compile software PMD backed by SZEDATA2 device
pub const CONFIG_RTE_LIBRTE_PMD_SZEDATA2: bool = false;
/// Compile burst-oriented Cavium Thunderx NICVF PMD driver
pub const CONFIG_RTE_LIBRTE_THUNDERX_NICVF_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_THUNDERX_NICVF_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_THUNDERX_NICVF_DEBUG_TX: bool = false;
/// Compile burst-oriented Cavium LiquidIO PMD driver
pub const CONFIG_RTE_LIBRTE_LIO_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_LIO_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_LIO_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_LIO_DEBUG_MBOX: bool = false;
pub const CONFIG_RTE_LIBRTE_LIO_DEBUG_REGS: bool = false;
/// Compile burst-oriented Cavium OCTEONTX network PMD driver
pub const CONFIG_RTE_LIBRTE_OCTEONTX_PMD: bool = true;
/// Compile WRS accelerated virtual port (AVP) guest PMD driver
pub const CONFIG_RTE_LIBRTE_AVP_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_AVP_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_AVP_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_AVP_DEBUG_BUFFERS: bool = false;
/// Compile burst-oriented VIRTIO PMD driver
pub const CONFIG_RTE_LIBRTE_VIRTIO_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_VIRTIO_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_VIRTIO_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_VIRTIO_DEBUG_DUMP: bool = false;
/// Compile virtio device emulation inside virtio PMD driver
pub const CONFIG_RTE_VIRTIO_USER: bool = true;
/// Compile burst-oriented VMXNET3 PMD driver
pub const CONFIG_RTE_LIBRTE_VMXNET3_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_VMXNET3_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_VMXNET3_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_VMXNET3_DEBUG_TX_FREE: bool = false;
/// Compile software PMD backed by AF_PACKET sockets (Linux only)
pub const CONFIG_RTE_LIBRTE_PMD_AF_PACKET: bool = true;
/// Compile link bonding PMD library
pub const CONFIG_RTE_LIBRTE_PMD_BOND: bool = true;
pub const CONFIG_RTE_LIBRTE_BOND_DEBUG_ALB: bool = false;
pub const CONFIG_RTE_LIBRTE_BOND_DEBUG_ALB_L1: bool = false;
/// Compile fail-safe PMD
pub const CONFIG_RTE_LIBRTE_PMD_FAILSAFE: bool = true;
/// Compile Marvell PMD driver
pub const CONFIG_RTE_LIBRTE_MVPP2_PMD: bool = false;
/// Compile Marvell MVNETA PMD driver
pub const CONFIG_RTE_LIBRTE_MVNETA_PMD: bool = false;
/// Compile support for VMBus library
pub const CONFIG_RTE_LIBRTE_VMBUS: bool = true;
/// Compile native PMD for Hyper-V/Azure
pub const CONFIG_RTE_LIBRTE_NETVSC_PMD: bool = true;
pub const CONFIG_RTE_LIBRTE_NETVSC_DEBUG_RX: bool = false;
pub const CONFIG_RTE_LIBRTE_NETVSC_DEBUG_TX: bool = false;
pub const CONFIG_RTE_LIBRTE_NETVSC_DEBUG_DUMP: bool = false;
/// Compile virtual device driver for NetVSC on Hyper-V/Azure
pub const CONFIG_RTE_LIBRTE_VDEV_NETVSC_PMD: bool = true;
/// Compile null PMD
pub const CONFIG_RTE_LIBRTE_PMD_NULL: bool = true;
/// Compile software PMD backed by PCAP files
pub const CONFIG_RTE_LIBRTE_PMD_PCAP: bool = false;
/// Compile example software rings based PMD
pub const CONFIG_RTE_LIBRTE_PMD_RING: bool = true;
pub const CONFIG_RTE_PMD_RING_MAX_RX_RINGS: u32 = 16;
pub const CONFIG_RTE_PMD_RING_MAX_TX_RINGS: u32 = 16;
/// Compile SOFTNIC PMD
pub const CONFIG_RTE_LIBRTE_PMD_SOFTNIC: bool = true;
/// Compile architecture we compile for. TAP PMD
/// It is enabled by default for Linux only.
pub const CONFIG_RTE_LIBRTE_PMD_TAP: bool = true;
/// Do prefetch of packet data within PMD driver receive function
pub const CONFIG_RTE_PMD_PACKET_PREFETCH: bool = true;
/// Compile generic wireless base band device library
/// EXPERIMENTAL: API may change without prior notice
pub const CONFIG_RTE_LIBRTE_BBDEV: bool = true;
pub const CONFIG_RTE_BBDEV_MAX_DEVS: u32 = 128;
pub const CONFIG_RTE_BBDEV_OFFLOAD_COST: bool = false;
/// Compile PMD for NULL bbdev device
pub const CONFIG_RTE_LIBRTE_PMD_BBDEV_NULL: bool = true;
/// Compile PMD for turbo software bbdev device
pub const CONFIG_RTE_LIBRTE_PMD_BBDEV_TURBO_SW: bool = false;
/// Compile generic crypto device library
pub const CONFIG_RTE_LIBRTE_CRYPTODEV: bool = true;
pub const CONFIG_RTE_CRYPTO_MAX_DEVS: u32 = 64;
/// Compile PMD for ARMv8 Crypto device
pub const CONFIG_RTE_LIBRTE_PMD_ARMV8_CRYPTO: bool = false;
pub const CONFIG_RTE_LIBRTE_PMD_ARMV8_CRYPTO_DEBUG: bool = false;
/// Compile NXP CAAM JR crypto Driver
pub const CONFIG_RTE_LIBRTE_PMD_CAAM_JR: bool = true;
pub const CONFIG_RTE_LIBRTE_PMD_CAAM_JR_BE: bool = false;
/// Compile NXP DPAA2 crypto sec driver for CAAM HW
pub const CONFIG_RTE_LIBRTE_PMD_DPAA2_SEC: bool = true;
/// NXP DPAA caam - crypto driver
pub const CONFIG_RTE_LIBRTE_PMD_DPAA_SEC: bool = true;
pub const CONFIG_RTE_LIBRTE_DPAA_MAX_CRYPTODEV: u32 = 4;
/// Compile PMD for Cavium OCTEON TX crypto device
pub const CONFIG_RTE_LIBRTE_PMD_OCTEONTX_CRYPTO: bool = true;
/// Compile PMD for QuickAssist based devices - see docs for details
pub const CONFIG_RTE_LIBRTE_PMD_QAT: bool = true;
pub const CONFIG_RTE_LIBRTE_PMD_QAT_SYM: bool = false;
/// Max. number of QuickAssist devices, which can be detected and attached
pub const CONFIG_RTE_PMD_QAT_MAX_PCI_DEVICES: u32 = 48;
pub const CONFIG_RTE_PMD_QAT_COMP_SGL_MAX_SEGMENTS: u32 = 16;
pub const CONFIG_RTE_PMD_QAT_COMP_IM_BUFFER_SIZE: u32 = 65536;
/// Compile PMD for virtio crypto devices
pub const CONFIG_RTE_LIBRTE_PMD_VIRTIO_CRYPTO: bool = true;
/// Number of maximum virtio crypto devices
pub const CONFIG_RTE_MAX_VIRTIO_CRYPTO: u32 = 32;
/// Compile PMD for AESNI backed device
pub const CONFIG_RTE_LIBRTE_PMD_AESNI_MB: bool = false;
/// Compile PMD for Software backed device
pub const CONFIG_RTE_LIBRTE_PMD_OPENSSL: bool = false;
/// Compile PMD for AESNI GCM device
pub const CONFIG_RTE_LIBRTE_PMD_AESNI_GCM: bool = false;
/// Compile PMD for SNOW 3G device
pub const CONFIG_RTE_LIBRTE_PMD_SNOW3G: bool = false;
pub const CONFIG_RTE_LIBRTE_PMD_SNOW3G_DEBUG: bool = false;
/// Compile PMD for KASUMI device
pub const CONFIG_RTE_LIBRTE_PMD_KASUMI: bool = false;
/// Compile PMD for ZUC device
pub const CONFIG_RTE_LIBRTE_PMD_ZUC: bool = false;
/// Compile PMD for Crypto Scheduler device
pub const CONFIG_RTE_LIBRTE_PMD_CRYPTO_SCHEDULER: bool = true;
/// Compile PMD for NULL Crypto device
pub const CONFIG_RTE_LIBRTE_PMD_NULL_CRYPTO: bool = true;
/// Compile PMD for AMD CCP crypto device
pub const CONFIG_RTE_LIBRTE_PMD_CCP: bool = false;
/// Compile PMD for Marvell Crypto device
pub const CONFIG_RTE_LIBRTE_PMD_MVSAM_CRYPTO: bool = false;
/// Compile generic security library
pub const CONFIG_RTE_LIBRTE_SECURITY: bool = true;
/// Compile generic compression device library
pub const CONFIG_RTE_LIBRTE_COMPRESSDEV: bool = true;
pub const CONFIG_RTE_COMPRESS_MAX_DEVS: u32 = 64;
/// Compile compressdev unit test
pub const CONFIG_RTE_COMPRESSDEV_TEST: bool = false;
/// Compile PMD for Octeontx ZIPVF compression device
pub const CONFIG_RTE_LIBRTE_PMD_OCTEONTX_ZIPVF: bool = true;
/// Compile PMD for ISA-L compression device
pub const CONFIG_RTE_LIBRTE_PMD_ISAL: bool = false;
/// Compile PMD for ZLIB compression device
pub const CONFIG_RTE_LIBRTE_PMD_ZLIB: bool = false;
/// Compile generic event device library
pub const CONFIG_RTE_LIBRTE_EVENTDEV: bool = true;
pub const CONFIG_RTE_LIBRTE_EVENTDEV_DEBUG: bool = false;
pub const CONFIG_RTE_EVENT_MAX_DEVS: u32 = 16;
pub const CONFIG_RTE_EVENT_MAX_QUEUES_PER_DEV: u32 = 64;
pub const CONFIG_RTE_EVENT_TIMER_ADAPTER_NUM_MAX: u32 = 32;
pub const CONFIG_RTE_EVENT_ETH_INTR_RING_SIZE: u32 = 1024;
pub const CONFIG_RTE_EVENT_CRYPTO_ADAPTER_MAX_INSTANCE: u32 = 32;
pub const CONFIG_RTE_EVENT_ETH_TX_ADAPTER_MAX_INSTANCE: u32 = 32;
/// Compile PMD for skeleton event device
pub const CONFIG_RTE_LIBRTE_PMD_SKELETON_EVENTDEV: bool = true;
pub const CONFIG_RTE_LIBRTE_PMD_SKELETON_EVENTDEV_DEBUG: bool = false;
/// Compile PMD for software event device
pub const CONFIG_RTE_LIBRTE_PMD_SW_EVENTDEV: bool = true;
/// Compile PMD for distributed software event device
pub const CONFIG_RTE_LIBRTE_PMD_DSW_EVENTDEV: bool = true;
/// Compile PMD for octeontx sso event device
pub const CONFIG_RTE_LIBRTE_PMD_OCTEONTX_SSOVF: bool = true;
/// Compile PMD for OPDL event device
pub const CONFIG_RTE_LIBRTE_PMD_OPDL_EVENTDEV: bool = true;
/// Compile PMD for NXP DPAA event device
pub const CONFIG_RTE_LIBRTE_PMD_DPAA_EVENTDEV: bool = true;
/// Compile PMD for NXP DPAA2 event device
pub const CONFIG_RTE_LIBRTE_PMD_DPAA2_EVENTDEV: bool = true;
/// Compile raw device support
/// EXPERIMENTAL: API may change without prior notice
pub const CONFIG_RTE_LIBRTE_RAWDEV: bool = true;
pub const CONFIG_RTE_RAWDEV_MAX_DEVS: u32 = 10;
pub const CONFIG_RTE_LIBRTE_PMD_SKELETON_RAWDEV: bool = true;
/// Compile PMD for NXP DPAA2 CMDIF raw device
pub const CONFIG_RTE_LIBRTE_PMD_DPAA2_CMDIF_RAWDEV: bool = true;
/// Compile PMD for NXP DPAA2 QDMA raw device
pub const CONFIG_RTE_LIBRTE_PMD_DPAA2_QDMA_RAWDEV: bool = true;
/// Compile PMD for Intel FPGA raw device
pub const CONFIG_RTE_LIBRTE_PMD_IFPGA_RAWDEV: bool = true;
/// Compile librte_ring
pub const CONFIG_RTE_LIBRTE_RING: bool = true;
/// Compile librte_mempool
pub const CONFIG_RTE_LIBRTE_MEMPOOL: bool = true;
pub const CONFIG_RTE_MEMPOOL_CACHE_MAX_SIZE: u32 = 512;
pub const CONFIG_RTE_LIBRTE_MEMPOOL_DEBUG: bool = false;
/// Compile Mempool drivers
pub const CONFIG_RTE_DRIVER_MEMPOOL_BUCKET: bool = true;
pub const CONFIG_RTE_DRIVER_MEMPOOL_BUCKET_SIZE_KB: u32 = 64;
pub const CONFIG_RTE_DRIVER_MEMPOOL_RING: bool = true;
pub const CONFIG_RTE_DRIVER_MEMPOOL_STACK: bool = true;
/// Compile PMD for octeontx fpa mempool device
pub const CONFIG_RTE_LIBRTE_OCTEONTX_MEMPOOL: bool = true;
/// Compile librte_mbuf
pub const CONFIG_RTE_LIBRTE_MBUF: bool = true;
pub const CONFIG_RTE_LIBRTE_MBUF_DEBUG: bool = false;
pub const CONFIG_RTE_MBUF_DEFAULT_MEMPOOL_OPS: &str = "ring_mp_mc";
pub const CONFIG_RTE_MBUF_REFCNT_ATOMIC: bool = true;
pub const CONFIG_RTE_PKTMBUF_HEADROOM: u32 = 128;
/// Compile librte_timer
pub const CONFIG_RTE_LIBRTE_TIMER: bool = true;
pub const CONFIG_RTE_LIBRTE_TIMER_DEBUG: bool = false;
/// Compile librte_cfgfile
pub const CONFIG_RTE_LIBRTE_CFGFILE: bool = true;
/// Compile librte_cmdline
pub const CONFIG_RTE_LIBRTE_CMDLINE: bool = true;
pub const CONFIG_RTE_LIBRTE_CMDLINE_DEBUG: bool = false;
/// Compile librte_hash
pub const CONFIG_RTE_LIBRTE_HASH: bool = true;
pub const CONFIG_RTE_LIBRTE_HASH_DEBUG: bool = false;
/// Compile librte_efd
pub const CONFIG_RTE_LIBRTE_EFD: bool = true;
/// Compile librte_member
pub const CONFIG_RTE_LIBRTE_MEMBER: bool = true;
/// Compile librte_jobstats
pub const CONFIG_RTE_LIBRTE_JOBSTATS: bool = true;
/// Compile architecture we compile for. device metrics library
pub const CONFIG_RTE_LIBRTE_METRICS: bool = true;
/// Compile architecture we compile for. bitrate statistics library
pub const CONFIG_RTE_LIBRTE_BITRATE: bool = true;
/// Compile architecture we compile for. latency statistics library
pub const CONFIG_RTE_LIBRTE_LATENCY_STATS: bool = true;
/// Compile librte_telemetry
pub const CONFIG_RTE_LIBRTE_TELEMETRY: bool = false;
/// Compile librte_lpm
pub const CONFIG_RTE_LIBRTE_LPM: bool = true;
pub const CONFIG_RTE_LIBRTE_LPM_DEBUG: bool = false;
/// Compile librte_acl
pub const CONFIG_RTE_LIBRTE_ACL: bool = true;
pub const CONFIG_RTE_LIBRTE_ACL_DEBUG: bool = false;
/// Compile librte_power
pub const CONFIG_RTE_LIBRTE_POWER: bool = true;
pub const CONFIG_RTE_LIBRTE_POWER_DEBUG: bool = false;
pub const CONFIG_RTE_MAX_LCORE_FREQS: u32 = 64;
/// Compile librte_net
pub const CONFIG_RTE_LIBRTE_NET: bool = true;
/// Compile librte_ip_frag
pub const CONFIG_RTE_LIBRTE_IP_FRAG: bool = true;
pub const CONFIG_RTE_LIBRTE_IP_FRAG_DEBUG: bool = false;
pub const CONFIG_RTE_LIBRTE_IP_FRAG_MAX_FRAG: u32 = 4;
pub const CONFIG_RTE_LIBRTE_IP_FRAG_TBL_STAT: bool = false;
/// Compile GRO library
pub const CONFIG_RTE_LIBRTE_GRO: bool = true;
/// Compile GSO library
pub const CONFIG_RTE_LIBRTE_GSO: bool = true;
/// Compile librte_meter
pub const CONFIG_RTE_LIBRTE_METER: bool = true;
/// Compile librte_classify
pub const CONFIG_RTE_LIBRTE_FLOW_CLASSIFY: bool = true;
/// Compile librte_sched
pub const CONFIG_RTE_LIBRTE_SCHED: bool = true;
pub const CONFIG_RTE_SCHED_DEBUG: bool = false;
pub const CONFIG_RTE_SCHED_RED: bool = false;
pub const CONFIG_RTE_SCHED_COLLECT_STATS: bool = false;
pub const CONFIG_RTE_SCHED_SUBPORT_TC_OV: bool = false;
pub const CONFIG_RTE_SCHED_PORT_N_GRINDERS: u32 = 8;
pub const CONFIG_RTE_SCHED_VECTOR: bool = false;
/// Compile architecture we compile for. distributor library
pub const CONFIG_RTE_LIBRTE_DISTRIBUTOR: bool = true;
/// Compile architecture we compile for. reorder library
pub const CONFIG_RTE_LIBRTE_REORDER: bool = true;
/// Compile librte_port
pub const CONFIG_RTE_LIBRTE_PORT: bool = true;
pub const CONFIG_RTE_PORT_STATS_COLLECT: bool = false;
pub const CONFIG_RTE_PORT_PCAP: bool = false;
/// Compile librte_table
pub const CONFIG_RTE_LIBRTE_TABLE: bool = true;
pub const CONFIG_RTE_TABLE_STATS_COLLECT: bool = false;
/// Compile librte_pipeline
pub const CONFIG_RTE_LIBRTE_PIPELINE: bool = true;
pub const CONFIG_RTE_PIPELINE_STATS_COLLECT: bool = false;
/// Compile librte_kni
pub const CONFIG_RTE_LIBRTE_KNI: bool = true;
pub const CONFIG_RTE_LIBRTE_PMD_KNI: bool = true;
pub const CONFIG_RTE_KNI_KMOD: bool = true;
pub const CONFIG_RTE_KNI_KMOD_ETHTOOL: bool = false;
pub const CONFIG_RTE_KNI_PREEMPT_DEFAULT: bool = true;
/// Compile architecture we compile for. pdump library
pub const CONFIG_RTE_LIBRTE_PDUMP: bool = true;
/// Compile vhost user library
pub const CONFIG_RTE_LIBRTE_VHOST: bool = true;
pub const CONFIG_RTE_LIBRTE_VHOST_NUMA: bool = true;
pub const CONFIG_RTE_LIBRTE_VHOST_DEBUG: bool = false;
/// Compile vhost PMD
/// To compile, CONFIG_RTE_LIBRTE_VHOST should be enabled.
pub const CONFIG_RTE_LIBRTE_PMD_VHOST: bool = true;
/// Compile IFC driver
/// To compile, CONFIG_RTE_LIBRTE_VHOST and CONFIG_RTE_EAL_VFIO
/// should be enabled.
pub const CONFIG_RTE_LIBRTE_IFC_PMD: bool = true;
/// Compile librte_bpf
pub const CONFIG_RTE_LIBRTE_BPF: bool = true;
/// allow load BPF from ELF files (requires libelf)
pub const CONFIG_RTE_LIBRTE_BPF_ELF: bool = false;
/// Compile architecture we compile for. test application
pub const CONFIG_RTE_APP_TEST: bool = true;
pub const CONFIG_RTE_APP_TEST_RESOURCE_TAR: bool = false;
/// Compile architecture we compile for. procinfo application
pub const CONFIG_RTE_PROC_INFO: bool = true;
/// Compile architecture we compile for. PMD test application
pub const CONFIG_RTE_TEST_PMD: bool = true;
pub const CONFIG_RTE_TEST_PMD_RECORD_CORE_CYCLES: bool = false;
pub const CONFIG_RTE_TEST_PMD_RECORD_BURST_STATS: bool = false;
/// Compile architecture we compile for. bbdev test application
pub const CONFIG_RTE_TEST_BBDEV: bool = true;
/// Compile architecture we compile for. crypto performance application
pub const CONFIG_RTE_APP_CRYPTO_PERF: bool = true;
/// Compile architecture we compile for. eventdev application
pub const CONFIG_RTE_APP_EVENTDEV: bool = true;
pub const CONFIG_RTE_EXEC_ENV_LINUXAPP: bool = true;
pub const CONFIG_RTE_LIBRTE_VHOST_POSTCOPY: bool = false;
/// Common libraries, before Bus/PMDs
/// NXP DPAA BUS and drivers
/// NXP FSLMC BUS and DPAA2 drivers
/// NXP ENETC PMD Driver
pub const CONFIG_RTE_ARCH_X86_64: bool = true;
pub const CONFIG_RTE_ARCH_X86: bool = true;
pub const CONFIG_RTE_ARCH_64: bool = true;
pub const CONFIG_RTE_TOOLCHAIN_GCC: bool = true;
