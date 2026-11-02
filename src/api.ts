import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import type {
  A2lProject,
  AddressChange,
  AddressMode,
  Characteristic,
  Measurement,
  SymbolNode,
  ValidationResult,
} from './types';

export const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (isTauri) {
    return invoke<T>(command, args);
  }
  const { mockInvoke } = await import('./mock');
  return mockInvoke(command, args) as T;
}

export const api = {
  newProject: () => call<A2lProject>('new_project'),
  loadProject: (path: string) => call<A2lProject>('load_project', { path }),
  saveProject: (path: string, project: A2lProject) => call<A2lProject>('save_project', { path, project }),
  loadSymbols: (path: string) => call<SymbolNode[]>('load_symbols', { path }),
  previewA2l: (project: A2lProject) => call<string>('preview_a2l', { project }),
  validateProject: (project: A2lProject) => call<ValidationResult>('validate_project', { project }),
  exportA2l: (path: string, project: A2lProject) => call<void>('export_a2l', { path, project }),
  detectAddressChanges: (project: A2lProject, symbols: SymbolNode[]) =>
    call<AddressChange[]>('detect_address_changes', { project, symbols }),
  applyAddressChanges: (project: A2lProject, changes: AddressChange[]) =>
    call<A2lProject>('apply_address_changes', { project, changes }),
  createMeasurement: (symbol: SymbolNode, addressMode: AddressMode) =>
    call<Measurement>('create_measurement', { symbol, addressMode }),
  createCharacteristic: (symbol: SymbolNode, addressMode: AddressMode) =>
    call<Characteristic>('create_characteristic', { symbol, addressMode }),
  fileExists: (path: string) => call<boolean>('file_exists', { path }),
};

interface DialogFilter {
  name: string;
  extensions: string[];
}

interface DialogOptions {
  filters?: DialogFilter[];
  defaultPath?: string;
}

function mockPath(options: DialogOptions, fallbackExtension: string): string {
  const extensions = options.filters?.flatMap((filter) => filter.extensions) ?? [];
  const extension = extensions.find((value) => value !== '*') ?? fallbackExtension;
  const baseName = options.defaultPath?.replace(/\\/g, '/').split('/').pop() ?? `Demo.${extension}`;
  return `mock://${baseName.endsWith(`.${extension}`) ? baseName : `Demo.${extension}`}`;
}

export async function openFileDialog(options: DialogOptions): Promise<string | null> {
  if (isTauri) {
    const result = await open({
      multiple: false,
      directory: false,
      filters: options.filters,
      defaultPath: options.defaultPath,
    });
    return typeof result === 'string' ? result : null;
  }
  return mockPath(options, 'out');
}

export async function saveFileDialog(options: DialogOptions): Promise<string | null> {
  if (isTauri) {
    return save({
      filters: options.filters,
      defaultPath: options.defaultPath,
    });
  }
  return mockPath(options, 'xml');
}
