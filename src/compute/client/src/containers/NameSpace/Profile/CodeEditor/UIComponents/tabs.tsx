import React, { useState } from 'react';
import {
  Tabs as BlueprintTabs,
  Tab,
  Navbar,
  Alignment,
  Button
} from '@blueprintjs/core';
import EditorWindow from '../LayoutComponents/Editor/EditorWindow';
import './tabs.scss';
export interface EditorTabs {
  id: string;
  filename: string;
}

interface FilesTabsProps {
  tabs: EditorTabs[];
  onCloseTab: (id: string) => void;
  setActiveTab: (id: string) => void;
  currentTabId: React.ReactText;
  betweenComponent?: React.ReactNode;
}

function useForceUpdate() {
  const [, setValue] = useState(0); // integer state
  return () => setValue((value) => ++value); // update the state to force render
}

export const FilesTabs = (props: FilesTabsProps) => {
  const forceUpdate = useForceUpdate();
  return (
    <>
      <Navbar style={{ height: 45 }}>
        <Navbar.Group
          align={Alignment.CENTER}
          style={{ overflowX: 'auto', overflowY: 'hidden' }}
        >
          <BlueprintTabs selectedTabId={props.currentTabId}>
            {props.tabs.map((tab, i) => (
              <Tab
                key={i}
                style={{
                  minWidth: 150,
                  maxWidth: 250,
                  textOverflow: 'ellipsis',
                  whiteSpace: 'nowrap'
                }}
                id={tab.id}
                title={
                  <div
                    style={{
                      display: 'flex',
                      flexDirection: 'row',
                      height: 40
                    }}
                  >
                    <Button
                      minimal
                      icon={'cross'}
                      style={{ borderRadius: 0, width: 35 }}
                      onClick={() => {
                        props.onCloseTab(tab.id);
                        forceUpdate();
                      }}
                    />
                    <div
                      style={{ marginTop: 3, fontSize: 15 }}
                      onClick={() => props.setActiveTab(tab.id)}
                    >
                      {tab.filename}
                    </div>
                  </div>
                }
              ></Tab>
            ))}
          </BlueprintTabs>
        </Navbar.Group>
      </Navbar>
      {props.betweenComponent}
      {props.tabs.map((tab) => (
        <div
          className="editor-wrap"
          key={tab.id}
          style={{ display: tab.id !== props.currentTabId ? 'none' : 'block' }}
        >
          <EditorWindow document={tab.filename} editor="Monaco" />
        </div>
      ))}
    </>
  );
};

export default FilesTabs;
