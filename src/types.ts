export const A2L_DATA_TYPES = [
  'UBYTE',
  'SBYTE',
  'UWORD',
  'SWORD',
  'ULONG',
  'SLONG',
  'A_UINT64',
  'A_INT64',
  'FLOAT32_IEEE',
  'FLOAT64_IEEE',
] as const;

export type A2lDataType = (typeof A2L_DATA_TYPES)[number];

export const ADDRESS_MODES = ['Raw', 'ByteAddress', 'WordAddress'] as const;
export type AddressMode = (typeof ADDRESS_MODES)[number];

export const CHARACTERISTIC_TYPES = [
  'VALUE',
  'CURVE',
  'MAP',
  'CUBOID',
  'CUBE_4',
  'CUBE_5',
  'VAL_BLK',
  'ASCII',
] as const;
export type CharacteristicType = (typeof CHARACTERISTIC_TYPES)[number];

export const PRG_TYPES = [
  'CALIBRATION_VARIABLES',
  'CODE',
  'DATA',
  'EXCLUDE_FROM_FLASH',
  'OFFLINE_DATA',
  'RESERVED',
  'SERAM',
  'VARIABLES',
] as const;
export type PrgType = (typeof PRG_TYPES)[number];

export const MEM_MEMORY_TYPES = [
  'EEPROM',
  'EPROM',
  'FLASH',
  'RAM',
  'ROM',
  'REGISTER',
  'NOT_IN_ECU',
] as const;
export type MemMemoryType = (typeof MEM_MEMORY_TYPES)[number];

export const MEM_ATTRIBUTES = ['INTERN', 'EXTERN'] as const;
export type MemAttribute = (typeof MEM_ATTRIBUTES)[number];

export const EVENT_TYPES = ['DAQ', 'STIM', 'DAQ_STIM'] as const;
export type EventType = (typeof EVENT_TYPES)[number];

export interface CompuMethod {
  name: string;
  kind: string;
  factor: number;
  offset: number;
  unit: string;
}

export interface Measurement {
  name: string;
  longIdentifier: string;
  ecuAddress: number;
  physUnit: string;
  format: string;
  lowerLimit: number;
  upperLimit: number;
  conversion: string;
  isEnabled: boolean;
  dataType: A2lDataType;
  resolution: number;
  accuracy: number;
}

export interface Characteristic {
  name: string;
  longIdentifier: string;
  ecuAddress: number;
  physUnit: string;
  format: string;
  lowerLimit: number;
  upperLimit: number;
  conversion: string;
  isEnabled: boolean;
  characteristicType: CharacteristicType;
  deposit: string;
  maxDiff: number;
  extendedLowerLimit: number;
  extendedUpperLimit: number;
}

export interface MemorySegment {
  name: string;
  longIdentifier: string;
  prgType: PrgType;
  memoryType: MemMemoryType;
  attribute: MemAttribute;
  address: number;
  size: number;
  offset0: number;
  offset1: number;
  offset2: number;
  offset3: number;
  offset4: number;
}

export interface XcpOptions {
  croCanId: number;
  dtoCanId: number;
  canBaudRate: number;
  protocolLayerVersion: number;
  t1: number;
  t2: number;
  t3: number;
  t4: number;
  t5: number;
  t6: number;
  t7: number;
  maxCto: number;
  maxDto: number;
  byteOrder: string;
  addressGranularity: number;
  canVersion: number;
  addressMode: AddressMode;
}

export interface DaqConfig {
  mode: string;
  maxDaq: number;
  maxEventChannel: number;
  minDaq: number;
  optimisationType: string;
  addressExtension: string;
  identificationFieldType: string;
  granularity: string;
  maxOdtEntrySizeDaq: number;
  overloadIndication: string;
  defaultMaxOdt: number;
  defaultMaxOdtEntries: number;
  prescalerSupported: boolean;
  resumeSupported: boolean;
  pidOffSupported: boolean;
  timestampSupported: boolean;
  timestampTicks: number;
  timestampSize: string;
  timestampUnit: string;
  timestampFixed: boolean;
  stimGranularity: string;
  maxOdtEntrySizeStim: number;
}

export interface EventChannel {
  name: string;
  shortName: string;
  channelNumber: number;
  type: string;
  maxDaqList: number;
  timeCycle: number;
  timeUnit: number;
  priority: number;
}

export interface OdtEntry {
  number: number;
  address: number;
  addressExtension: number;
  size: number;
  bitOffset: number;
}

export interface OdtConfig {
  number: number;
  entries: OdtEntry[];
}

export interface DaqListConfig {
  number: number;
  type: string;
  maxOdt: number;
  maxOdtEntries: number;
  firstPid: number;
  eventFixed: number;
  odts: OdtConfig[];
}

export interface A2lProject {
  projectName: string;
  moduleName: string;
  outFilePath: string;
  targetPlatform: string;
  xcp: XcpOptions;
  daq: DaqConfig;
  eventChannels: EventChannel[];
  daqLists: DaqListConfig[];
  measurements: Measurement[];
  characteristics: Characteristic[];
  memorySegments: MemorySegment[];
  compuMethods: CompuMethod[];
  lastExportPath: string;
}

export interface SymbolNode {
  name: string;
  path: string;
  address: number;
  size: number;
  ctype: string;
  sourceFile: string;
  sourceLine: number | null;
  isLeaf: boolean;
  requiresTypeConfirmation: boolean;
  suggestedDataType: A2lDataType;
  children: SymbolNode[];
}

export interface AddressChange {
  name: string;
  oldAddress: number;
  newAddress: number;
}

export interface ValidationResult {
  errors: string[];
  warnings: string[];
  canExport: boolean;
}

export interface TreeDataNode {
  key: string;
  title: string;
  isLeaf: boolean;
  children?: TreeDataNode[];
}
