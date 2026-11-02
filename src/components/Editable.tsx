import { Input, Select } from 'antd';
import { useState } from 'react';
import { formatHex, parseHex } from '../format';

function formatNumber(value: number): string {
  return Number.isFinite(value) ? String(value) : '';
}

function commitOnEnter(event: React.KeyboardEvent<HTMLInputElement>) {
  event.currentTarget.blur();
}

interface EditableTextProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
}

export function EditableText({ value, onChange, placeholder }: EditableTextProps) {
  const [draft, setDraft] = useState<string | null>(null);

  const commit = () => {
    const next = (draft ?? value).trim();
    setDraft(null);
    if (next !== value) onChange(next);
  };

  return (
    <Input
      size="small"
      value={draft ?? value}
      placeholder={placeholder}
      onChange={(event) => setDraft(event.target.value)}
      onBlur={commit}
      onPressEnter={commitOnEnter}
    />
  );
}

interface EditableNumberProps {
  value: number;
  onChange: (value: number) => void;
  integer?: boolean;
  min?: number;
  max?: number;
}

export function EditableNumber({ value, onChange, integer, min, max }: EditableNumberProps) {
  const [draft, setDraft] = useState<string | null>(null);

  const commit = () => {
    const text = draft ?? formatNumber(value);
    const parsed = integer ? Number.parseInt(text, 10) : Number.parseFloat(text);
    setDraft(null);
    if (!Number.isFinite(parsed)) return;

    let next = parsed;
    if (min !== undefined) next = Math.max(min, next);
    if (max !== undefined) next = Math.min(max, next);
    if (integer) next = Math.trunc(next);
    if (next !== value) onChange(next);
  };

  return (
    <Input
      size="small"
      value={draft ?? formatNumber(value)}
      inputMode="decimal"
      onChange={(event) => setDraft(event.target.value)}
      onBlur={commit}
      onPressEnter={commitOnEnter}
    />
  );
}

interface EditableOptionalNumberProps {
  value: number;
  onChange: (value: number) => void;
  emptyValue: number;
}

export function EditableOptionalNumber({ value, onChange, emptyValue }: EditableOptionalNumberProps) {
  const [draft, setDraft] = useState<string | null>(null);
  const isSentinel = Math.abs(value) === Number.MAX_VALUE;
  const display = draft ?? (isSentinel ? '' : formatNumber(value));

  const commit = () => {
    const text = (draft ?? display).trim();
    setDraft(null);
    if (!text) {
      if (value !== emptyValue) onChange(emptyValue);
      return;
    }
    const parsed = Number.parseFloat(text);
    if (Number.isFinite(parsed) && parsed !== value) onChange(parsed);
  };

  return (
    <Input
      size="small"
      value={display}
      placeholder="默认"
      onChange={(event) => setDraft(event.target.value)}
      onBlur={commit}
      onPressEnter={commitOnEnter}
    />
  );
}

interface EditableHexProps {
  value: number;
  onChange: (value: number) => void;
}

export function EditableHex({ value, onChange }: EditableHexProps) {
  const [draft, setDraft] = useState<string | null>(null);

  const commit = () => {
    const parsed = parseHex(draft ?? formatHex(value));
    setDraft(null);
    if (parsed !== null && parsed !== value) onChange(parsed);
  };

  return (
    <Input
      size="small"
      value={draft ?? formatHex(value)}
      onChange={(event) => setDraft(event.target.value)}
      onBlur={commit}
      onPressEnter={commitOnEnter}
      style={{ fontFamily: 'Cascadia Code, Consolas, monospace' }}
    />
  );
}

interface EditableSelectProps<T extends string | number> {
  value: T;
  options: readonly { label: string; value: T }[];
  onChange: (value: T) => void;
}

export function EditableSelect<T extends string | number>({ value, options, onChange }: EditableSelectProps<T>) {
  return (
    <Select
      size="small"
      value={value}
      options={options as { label: string; value: T }[]}
      onChange={onChange}
      style={{ width: '100%' }}
    />
  );
}

interface HexInputProps {
  value?: number;
  onChange?: (value: number) => void;
  placeholder?: string;
}

export function HexInput({ value, onChange, placeholder }: HexInputProps) {
  const [draft, setDraft] = useState<string | null>(null);
  const display = draft ?? (value !== undefined ? formatHex(value) : '');

  const commit = () => {
    const parsed = parseHex(draft ?? display);
    setDraft(null);
    if (parsed === null) return;
    onChange?.(parsed);
  };

  return (
    <Input
      value={display}
      placeholder={placeholder}
      onChange={(event) => setDraft(event.target.value)}
      onBlur={commit}
      onPressEnter={commitOnEnter}
      style={{ fontFamily: 'Cascadia Code, Consolas, monospace' }}
    />
  );
}
