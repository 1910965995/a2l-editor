import { PlusOutlined, SearchOutlined } from '@ant-design/icons';
import { Button, Empty, Input, Tree } from 'antd';
import { useMemo } from 'react';
import { useAppStore } from '../store';
import type { SymbolNode } from '../types';

interface TreeItem {
  key: string;
  title: string;
  isLeaf?: boolean;
  children?: TreeItem[];
}

function filterTree(nodes: SymbolNode[], query: string): SymbolNode[] {
  if (!query) return nodes;
  const needle = query.toLowerCase();
  const result: SymbolNode[] = [];

  for (const node of nodes) {
    if (node.children.length === 0) {
      if (node.name.toLowerCase().includes(needle)) result.push(node);
      continue;
    }

    const children = filterTree(node.children, query);
    if (children.length > 0) {
      result.push({ ...node, children });
    } else if (node.name.toLowerCase().includes(needle)) {
      result.push(node);
    }
  }

  return result;
}

function toTreeItems(nodes: SymbolNode[]): TreeItem[] {
  return nodes.map((node) => ({
    key: node.path,
    title: node.name,
    isLeaf: node.isLeaf,
    children: node.children.length > 0 ? toTreeItems(node.children) : undefined,
  }));
}

export function SymbolTree() {
  const symbols = useAppStore((state) => state.symbols);
  const searchText = useAppStore((state) => state.searchText);
  const setSearchText = useAppStore((state) => state.setSearchText);
  const selectedSymbol = useAppStore((state) => state.selectedSymbol);
  const setSelectedSymbol = useAppStore((state) => state.setSelectedSymbol);
  const beginAddMeasurement = useAppStore((state) => state.beginAddMeasurement);
  const beginAddCharacteristic = useAppStore((state) => state.beginAddCharacteristic);

  const nodeMap = useMemo(() => {
    const map = new Map<string, SymbolNode>();
    const walk = (nodes: SymbolNode[]) => {
      for (const node of nodes) {
        map.set(node.path, node);
        walk(node.children);
      }
    };
    walk(symbols);
    return map;
  }, [symbols]);

  const visibleSymbols = useMemo(() => filterTree(symbols, searchText.trim()), [symbols, searchText]);
  const treeData = useMemo(() => toTreeItems(visibleSymbols), [visibleSymbols]);

  return (
    <div className="symbol-panel">
      <Input
        allowClear
        prefix={<SearchOutlined style={{ color: '#94a3b8' }} />}
        placeholder="搜索符号"
        value={searchText}
        onChange={(event) => setSearchText(event.target.value)}
      />

      <div className="symbol-tree-scroll">
        {treeData.length > 0 ? (
          <Tree
            blockNode
            treeData={treeData}
            selectedKeys={selectedSymbol ? [selectedSymbol.path] : []}
            onSelect={(keys) => {
              const key = String(keys[0] ?? '');
              setSelectedSymbol(nodeMap.get(key) ?? null);
            }}
          />
        ) : (
          <Empty
            image={Empty.PRESENTED_IMAGE_SIMPLE}
            description="暂无符号，请加载 ELF/DWARF"
            style={{ marginTop: 40 }}
          />
        )}
      </div>

      <div className="symbol-actions">
        <Button
          size="small"
          icon={<PlusOutlined />}
          disabled={!selectedSymbol?.isLeaf}
          onClick={() => void beginAddMeasurement()}
        >
          测量量
        </Button>
        <Button
          size="small"
          icon={<PlusOutlined />}
          disabled={!selectedSymbol?.isLeaf}
          onClick={() => void beginAddCharacteristic()}
        >
          标定量
        </Button>
      </div>
    </div>
  );
}
