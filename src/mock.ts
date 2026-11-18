import { defaultProject } from './constants';
import type {
  A2lDataType,
  A2lProject,
  AddressMode,
  Characteristic,
  Measurement,
  SymbolNode,
  ValidationResult,
} from './types';

const LIMITS: Record<A2lDataType, [number, number]> = {
  UBYTE: [0, 255],
  SBYTE: [-128, 127],
  UWORD: [0, 65535],
  SWORD: [-32768, 32767],
  ULONG: [0, 4294967295],
  SLONG: [-2147483648, 2147483647],
  A_UINT64: [0, 2 ** 64],
  A_INT64: [-(2 ** 63), 2 ** 63],
  FLOAT32_IEEE: [-3.4028234663852886e38, 3.4028234663852886e38],
  FLOAT64_IEEE: [-Number.MAX_VALUE, Number.MAX_VALUE],
};

function demoSymbols(): SymbolNode[] {
  const leaf = (
    name: string,
    path: string,
    address: number,
    size: number,
    dataType: A2lDataType,
    ctype: string,
  ): SymbolNode => ({
    name,
    path,
    address,
    size,
    ctype,
    sourceFile: 'motor.c',
    sourceLine: 12,
    isLeaf: true,
    requiresTypeConfirmation: false,
    suggestedDataType: dataType,
    children: [],
  });

  return [
    {
      name: 'motor',
      path: 'motor',
      address: 0xa000,
      size: 12,
      ctype: 'MotorState',
      sourceFile: 'motor.c',
      sourceLine: 8,
      isLeaf: false,
      requiresTypeConfirmation: false,
      suggestedDataType: 'UWORD',
      children: [
        leaf('speed', 'motor.speed', 0xa000, 2, 'UWORD', 'unsigned int'),
        leaf('state', 'motor.state', 0xa002, 2, 'SWORD', 'int'),
        leaf('gain', 'motor.gain', 0xa004, 4, 'FLOAT32_IEEE', 'float'),
        leaf('temperature', 'motor.temperature', 0xa008, 4, 'ULONG', 'unsigned long'),
      ],
    },
    {
      name: 'samples',
      path: 'samples',
      address: 0x9000,
      size: 6,
      ctype: '',
      sourceFile: 'motor.c',
      sourceLine: 20,
      isLeaf: false,
      requiresTypeConfirmation: false,
      suggestedDataType: 'UWORD',
      children: [0, 1, 2].map((index) =>
        leaf(`samples[${index}]`, `samples[${index}]`, 0x9000 + index * 2, 2, 'UWORD', 'unsigned int'),
      ),
    },
    leaf('temperature', 'temperature', 0xb000, 4, 'FLOAT32_IEEE', 'float'),
  ];
}

function demoProject(): A2lProject {
  const project = defaultProject();
  project.projectName = 'Demo';
  project.moduleName = 'ECU';
  project.outFilePath = 'D:\\workspace\\demo\\app.out';
  project.eventChannels = [
    {
      name: 'NewEvent1',
      shortName: 'NewEvent1',
      channelNumber: 0,
      type: 'DAQ',
      maxDaqList: 1,
      timeCycle: 10,
      timeUnit: 6,
      priority: 0,
    },
  ];
  project.daqLists = [
    {
      number: 0,
      type: 'DAQ',
      maxOdt: 2,
      maxOdtEntries: 4,
      firstPid: 0,
      eventFixed: 0,
      odts: [
        {
          number: 0,
          entries: [
            { number: 0, address: 0xa000, addressExtension: 0, size: 2, bitOffset: 0 },
            { number: 1, address: 0xa002, addressExtension: 0, size: 2, bitOffset: 0 },
          ],
        },
      ],
    },
  ];
  project.measurements = [
    {
      name: 'motor.speed',
      longIdentifier: '电机转速',
      ecuAddress: 0xa000,
      physUnit: 'rpm',
      format: '%6.2',
      lowerLimit: 0,
      upperLimit: 8000,
      conversion: 'IDENTICAL',
      isEnabled: true,
      dataType: 'UWORD',
      resolution: 1,
      accuracy: 0,
    },
    {
      name: 'motor.temperature',
      longIdentifier: '电机温度',
      ecuAddress: 0xa008,
      physUnit: 'degC',
      format: '%6.2',
      lowerLimit: -40,
      upperLimit: 150,
      conversion: 'IDENTICAL',
      isEnabled: true,
      dataType: 'FLOAT32_IEEE',
      resolution: 0.1,
      accuracy: 0,
    },
  ];
  project.characteristics = [
    {
      name: 'motor.gain',
      longIdentifier: '增益标定',
      ecuAddress: 0xa004,
      physUnit: '',
      format: '%6.2',
      lowerLimit: 0,
      upperLimit: 10,
      conversion: 'IDENTICAL',
      isEnabled: true,
      characteristicType: 'VALUE',
      deposit: '__FLOAT32_IEEE',
      maxDiff: 0,
      extendedLowerLimit: -Number.MAX_VALUE,
      extendedUpperLimit: Number.MAX_VALUE,
    },
  ];
  project.memorySegments = [
    {
      name: 'FLASH0',
      longIdentifier: '程序 Flash',
      prgType: 'CODE',
      memoryType: 'FLASH',
      attribute: 'INTERN',
      address: 0x8000,
      size: 0x10000,
      offset0: -1,
      offset1: -1,
      offset2: -1,
      offset3: -1,
      offset4: -1,
    },
  ];
  return project;
}

function hex(value: number): string {
  return `0x${value.toString(16).toUpperCase()}`;
}

function mockPreview(project: A2lProject): string {
  const lines: string[] = [
    'ASAP2_VERSION 1 71',
    `/begin PROJECT ${project.projectName} "${project.projectName}"`,
    `  /begin MODULE ${project.moduleName} "${project.moduleName}"`,
    '    /begin MOD_COMMON ""',
    `      BYTE_ORDER ${project.xcp.byteOrder}`,
    '      ALIGNMENT_BYTE 1',
    '    /end MOD_COMMON',
    '    /begin IF_DATA XCP',
    '      /begin PROTOCOL_LAYER',
    `        ${hex(project.xcp.protocolLayerVersion)}`,
    `        BYTE_ORDER_${project.xcp.byteOrder}`,
    '      /end PROTOCOL_LAYER',
    '      /begin XCP_ON_CAN',
    `        CAN_ID_MASTER ${hex(project.xcp.croCanId)}`,
    `        CAN_ID_SLAVE ${hex(project.xcp.dtoCanId)}`,
    '      /end XCP_ON_CAN',
    '    /end IF_DATA',
  ];

  for (const measurement of project.measurements.filter((item) => item.isEnabled)) {
    lines.push(`    /begin MEASUREMENT ${measurement.name} "${measurement.longIdentifier}"`);
    lines.push(
      `      ${measurement.dataType} ${measurement.conversion} ${measurement.resolution} ${measurement.accuracy} ${measurement.lowerLimit} ${measurement.upperLimit}`,
    );
    if (measurement.ecuAddress) lines.push(`      ECU_ADDRESS ${hex(measurement.ecuAddress)}`);
    lines.push('    /end MEASUREMENT');
  }

  for (const characteristic of project.characteristics.filter((item) => item.isEnabled)) {
    lines.push(`    /begin CHARACTERISTIC ${characteristic.name} "${characteristic.longIdentifier}"`);
    lines.push(
      `      ${characteristic.characteristicType} ${hex(characteristic.ecuAddress)} ${characteristic.deposit} ${characteristic.maxDiff} ${characteristic.conversion} ${characteristic.lowerLimit} ${characteristic.upperLimit}`,
    );
    lines.push('    /end CHARACTERISTIC');
  }

  for (const segment of project.memorySegments) {
    lines.push(`    /begin MEMORY_SEGMENT ${segment.name} "${segment.longIdentifier}"`);
    lines.push(
      `      ${segment.prgType} ${segment.memoryType} ${segment.attribute} ${hex(segment.address)} ${hex(segment.size)}`,
    );
    lines.push('    /end MEMORY_SEGMENT');
  }

  lines.push('  /end MODULE', '/end PROJECT');
  return lines.join('\n');
}

function mockValidate(project: A2lProject): ValidationResult {
  const errors: string[] = [];
  const warnings: string[] = [];

  if (!project.projectName.trim()) errors.push('项目名不能为空。');
  if (!project.moduleName.trim()) errors.push('模块名不能为空。');
  if (project.xcp.croCanId === 0) errors.push('CRO / Master CAN ID 不能为空。');
  if (project.xcp.dtoCanId === 0) errors.push('DTO / Slave CAN ID 不能为空。');

  const measurements = project.measurements.filter((item) => item.isEnabled);
  const characteristics = project.characteristics.filter((item) => item.isEnabled);
  if (measurements.length + characteristics.length === 0) errors.push('至少需要一个 A2L 对象。');

  const compuNames = new Set(project.compuMethods.map((item) => item.name.toLowerCase()));
  for (const measurement of measurements) {
    if (!compuNames.has(measurement.conversion.toLowerCase())) {
      errors.push(`测量量 ${measurement.name} 转换公式不存在: ${measurement.conversion}`);
    }
    if (!measurement.physUnit.trim()) warnings.push(`测量量 ${measurement.name} 单位为空。`);
    if (!measurement.longIdentifier.trim()) warnings.push(`测量量 ${measurement.name} 描述为空。`);
    if (measurement.ecuAddress === 0) warnings.push(`测量量 ${measurement.name} 地址为空（可选，但建议填写）。`);
  }

  for (const characteristic of characteristics) {
    if (!characteristic.ecuAddress) errors.push(`标定量 ${characteristic.name} 地址不能为空。`);
    if (!characteristic.deposit.trim()) {
      errors.push(`标定量 ${characteristic.name} Deposit (RECORD_LAYOUT 引用) 不能为空。`);
    }
    if (!compuNames.has(characteristic.conversion.toLowerCase())) {
      errors.push(`标定量 ${characteristic.name} 转换公式不存在: ${characteristic.conversion}`);
    }
    if (!characteristic.physUnit.trim()) warnings.push(`标定量 ${characteristic.name} 单位为空。`);
    if (!characteristic.longIdentifier.trim()) warnings.push(`标定量 ${characteristic.name} 描述为空。`);
  }

  if (project.xcp.addressMode === 'Raw') {
    warnings.push('请确认ELF文件地址类型：ELF文件地址是按字节编址还是按字编址。');
  }

  return { errors, warnings, canExport: errors.length === 0 };
}

function toEcuAddress(address: number, mode: AddressMode): number {
  return mode === 'WordAddress' ? address * 2 : address;
}

export async function mockInvoke(command: string, args?: Record<string, unknown>): Promise<unknown> {
  switch (command) {
    case 'new_project':
      return demoProject();
    case 'load_project':
      return { ...demoProject(), projectName: 'Demo Project' };
    case 'save_project':
      return args?.project;
    case 'load_symbols':
      return demoSymbols();
    case 'preview_a2l':
      return mockPreview(args?.project as A2lProject);
    case 'validate_project':
      return mockValidate(args?.project as A2lProject);
    case 'export_a2l':
      return undefined;
    case 'detect_address_changes':
      return [];
    case 'apply_address_changes':
      return args?.project;
    case 'create_measurement': {
      const symbol = args?.symbol as SymbolNode;
      const mode = args?.addressMode as AddressMode;
      return {
        ...({
          name: symbol.path,
          longIdentifier: '',
          ecuAddress: toEcuAddress(symbol.address, mode),
          physUnit: '',
          format: '%6.2',
          lowerLimit: LIMITS[symbol.suggestedDataType][0],
          upperLimit: LIMITS[symbol.suggestedDataType][1],
          conversion: 'IDENTICAL',
          isEnabled: true,
          dataType: symbol.suggestedDataType,
          resolution: 1,
          accuracy: 0,
        } satisfies Measurement),
      };
    }
    case 'create_characteristic': {
      const symbol = args?.symbol as SymbolNode;
      const mode = args?.addressMode as AddressMode;
      return {
        name: symbol.path,
        longIdentifier: '',
        ecuAddress: toEcuAddress(symbol.address, mode),
        physUnit: '',
        format: '%6.2',
        lowerLimit: LIMITS[symbol.suggestedDataType][0],
        upperLimit: LIMITS[symbol.suggestedDataType][1],
        conversion: 'IDENTICAL',
        isEnabled: true,
        characteristicType: 'VALUE',
        deposit: `__${symbol.suggestedDataType}`,
        maxDiff: 0,
        extendedLowerLimit: LIMITS[symbol.suggestedDataType][0],
        extendedUpperLimit: LIMITS[symbol.suggestedDataType][1],
      } satisfies Characteristic;
    }
    case 'file_exists':
      return true;
    default:
      throw new Error(`未实现的 mock 命令：${command}`);
  }
}
