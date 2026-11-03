import type {
  A2lProject,
  AddressMode,
  DaqConfig,
  MemorySegment,
  XcpOptions,
} from './types';

export const APP_TITLE = 'A2L Editor';

export const BYTE_ORDERS = ['MSB_LAST', 'MSB_FIRST'] as const;

export const ADDRESS_GRANULARITY_OPTIONS = [
  { label: 'BYTE', value: 1 },
  { label: 'WORD', value: 2 },
  { label: 'DWORD', value: 4 },
] as const;

export const ADDRESS_MODE_OPTIONS: { label: string; value: AddressMode }[] = [
  { label: 'RAW', value: 'Raw' },
  { label: 'BYTE', value: 'ByteAddress' },
  { label: 'WORD', value: 'WordAddress' },
];

export const DAQ_MODE_OPTIONS = [
  { label: '动态', value: 'DYNAMIC' },
  { label: '静态', value: 'STATIC' },
] as const;

export const OVERLOAD_OPTIONS = [
  { label: 'NONE', value: 'NO_OVERLOAD_INDICATION' },
  { label: 'PID', value: 'OVERLOAD_INDICATION_PID' },
  { label: 'EVENT', value: 'OVERLOAD_INDICATION_EVENT' },
] as const;

export const DAQ_GRANULARITY_OPTIONS = [
  { label: 'BYTE', value: 'GRANULARITY_ODT_ENTRY_SIZE_DAQ_BYTE' },
  { label: 'WORD', value: 'GRANULARITY_ODT_ENTRY_SIZE_DAQ_WORD' },
  { label: 'DWORD', value: 'GRANULARITY_ODT_ENTRY_SIZE_DAQ_DWORD' },
  { label: 'DLONG', value: 'GRANULARITY_ODT_ENTRY_SIZE_DAQ_DLONG' },
] as const;

export const ID_FIELD_OPTIONS = [
  { label: 'Absolute', value: 'IDENTIFICATION_FIELD_TYPE_ABSOLUTE' },
  { label: 'Relative Byte', value: 'IDENTIFICATION_FIELD_TYPE_RELATIVE_BYTE' },
  { label: 'Relative Word', value: 'IDENTIFICATION_FIELD_TYPE_RELATIVE_WORD' },
  { label: 'Relative Word Aligned', value: 'IDENTIFICATION_FIELD_TYPE_RELATIVE_WORD_ALIGNED' },
] as const;

export const ADDRESS_EXTENSION_OPTIONS = [
  { label: 'Free', value: 'ADDRESS_EXTENSION_FREE' },
  { label: 'ODT', value: 'ADDRESS_EXTENSION_ODT' },
  { label: 'DAQ', value: 'ADDRESS_EXTENSION_DAQ' },
] as const;

export const OPTIMISATION_OPTIONS = [
  { label: 'Default', value: 'OPTIMISATION_TYPE_DEFAULT' },
  { label: 'ODT Type 16', value: 'OPTIMISATION_TYPE_ODT_TYPE_16' },
  { label: 'ODT Type 32', value: 'OPTIMISATION_TYPE_ODT_TYPE_32' },
  { label: 'ODT Type 64', value: 'OPTIMISATION_TYPE_ODT_TYPE_64' },
  { label: 'Alignment', value: 'OPTIMISATION_TYPE_ODT_TYPE_ALIGNMENT' },
  { label: 'Max Entry Size', value: 'OPTIMISATION_TYPE_MAX_ENTRY_SIZE' },
] as const;

export const TIMESTAMP_SIZE_OPTIONS = ['NO_TIME_STAMP', 'SIZE_BYTE', 'SIZE_WORD', 'SIZE_DWORD'] as const;

export const TIMESTAMP_UNIT_OPTIONS = [
  'UNIT_1NS',
  'UNIT_10NS',
  'UNIT_100NS',
  'UNIT_1US',
  'UNIT_10US',
  'UNIT_100US',
  'UNIT_1MS',
  'UNIT_10MS',
  'UNIT_100MS',
  'UNIT_1S',
] as const;

export const TIME_UNIT_LABELS = [
  '1 ns',
  '10 ns',
  '100 ns',
  '1 us',
  '10 us',
  '100 us',
  '1 ms',
  '10 ms',
  '100 ms',
  '1 s',
];

export const OUT_FILE_FILTERS = [
  { name: 'TI CCS 输出文件', extensions: ['out'] },
  { name: 'ELF 文件', extensions: ['elf'] },
  { name: '所有文件', extensions: ['*'] },
];

export const PROJECT_FILE_FILTERS = [{ name: 'A2L Editor 工程', extensions: ['a2lproj'] }];
export const A2L_FILE_FILTERS = [{ name: 'A2L 文件', extensions: ['a2l'] }];

export function defaultDaqConfig(): DaqConfig {
  return {
    mode: 'DYNAMIC',
    maxDaq: 0,
    maxEventChannel: 5,
    minDaq: 0,
    optimisationType: 'OPTIMISATION_TYPE_DEFAULT',
    addressExtension: 'ADDRESS_EXTENSION_FREE',
    identificationFieldType: 'IDENTIFICATION_FIELD_TYPE_RELATIVE_BYTE',
    granularity: 'GRANULARITY_ODT_ENTRY_SIZE_DAQ_BYTE',
    maxOdtEntrySizeDaq: 5,
    overloadIndication: 'NO_OVERLOAD_INDICATION',
    defaultMaxOdt: 1,
    defaultMaxOdtEntries: 1,
    prescalerSupported: false,
    resumeSupported: false,
    pidOffSupported: false,
    timestampSupported: true,
    timestampTicks: 0,
    timestampSize: 'SIZE_DWORD',
    timestampUnit: 'UNIT_1MS',
    timestampFixed: true,
    stimGranularity: 'GRANULARITY_ODT_ENTRY_SIZE_STIM_BYTE',
    maxOdtEntrySizeStim: 6,
  };
}

export function defaultXcpOptions(): XcpOptions {
  return {
    croCanId: 0x720,
    dtoCanId: 0x700,
    canBaudRate: 500000,
    protocolLayerVersion: 0x0100,
    t1: 0,
    t2: 0,
    t3: 0,
    t4: 0,
    t5: 0,
    t6: 0,
    t7: 0,
    maxCto: 8,
    maxDto: 8,
    byteOrder: 'MSB_LAST',
    addressGranularity: 1,
    canVersion: 0x0100,
    addressMode: 'Raw',
  };
}

export function defaultProject(): A2lProject {
  return {
    projectName: 'Project',
    moduleName: 'Module',
    outFilePath: '',
    targetPlatform: 'TI C2000 / C28x',
    xcp: defaultXcpOptions(),
    daq: defaultDaqConfig(),
    eventChannels: [],
    daqLists: [],
    measurements: [],
    characteristics: [],
    memorySegments: [],
    compuMethods: [{ name: 'IDENTICAL', kind: 'IDENTICAL', factor: 1, offset: 0, unit: '' }],
    lastExportPath: '',
  };
}

export function defaultMemorySegment(): MemorySegment {
  return {
    name: '',
    longIdentifier: '',
    prgType: 'DATA',
    memoryType: 'FLASH',
    attribute: 'INTERN',
    address: 0,
    size: 0,
    offset0: -1,
    offset1: -1,
    offset2: -1,
    offset3: -1,
    offset4: -1,
  };
}

export const DATA_TYPE_SIZE: Record<string, number> = {
  UBYTE: 1,
  SBYTE: 1,
  UWORD: 2,
  SWORD: 2,
  ULONG: 4,
  SLONG: 4,
  A_UINT64: 8,
  A_INT64: 8,
  FLOAT32_IEEE: 4,
  FLOAT64_IEEE: 8,
};

export function depositSize(deposit: string): number {
  const name = deposit.startsWith('__') ? deposit.slice(2) : deposit;
  return DATA_TYPE_SIZE[name] ?? 2;
}

export function characteristicSize(characteristicType: string, deposit: string): number {
  return characteristicType === 'VALUE' ? depositSize(deposit) : 2;
}
