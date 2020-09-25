import React from 'react';
import MiddleContainer from '@/containers/Layout/MiddleContainer';
import { Result, Tree } from 'antd';
import { EventDataNode, DataNode } from 'rc-tree/lib/interface';
import { useProjectFolders } from '../hooks';
import { FileNodeInfo } from '@/types/store/project';
import { useStoreActions } from '@/store/hooks';

const { DirectoryTree } = Tree;

export default function FolderTree(props: {
  onSelectFile: (node: FileNodeInfo) => void;
}) {
  const [projectFolders, loading, error] = useProjectFolders();
  const setActiveNode = useStoreActions(
    (actions) => actions.project.setActiveNode
  );

  const onSelect = (
    keys: (string | number)[],
    event: {
      event: 'select';
      selected: boolean;
      node: EventDataNode;
      selectedNodes: DataNode[];
      nativeEvent: MouseEvent;
    }
  ) => {
    console.log('Trigger Select', keys, event);
    //@ts-ignore
    setActiveNode(event.node);
    //@ts-ignore
    props.onSelectFile(event.node);
  };

  const onExpand = () => {
    console.log('Trigger Expand');
  };

  if (loading || error) {
    return (
      <MiddleContainer loading={loading} error={<Result {...error?.props} />} />
    );
  }

  return (
    <DirectoryTree
      multiple
      defaultExpandAll
      onSelect={onSelect}
      onExpand={onExpand}
      treeData={projectFolders}
    />
  );
}
