export function formatHex(value: number): string {
  return `0x${value.toString(16).toUpperCase()}`;
}

export function parseHex(text: string): number | null {
  const trimmed = text.trim();
  if (!trimmed) return null;
  const parsed = /^0x/i.test(trimmed)
    ? Number.parseInt(trimmed.slice(2), 16)
    : Number.parseInt(trimmed, 10);
  return Number.isFinite(parsed) ? parsed : null;
}
