import React, { Component } from 'react';
import EditorLauncher from './EditorLauncher';
export interface EditorWindowState {
  editor: string | null;
  document: string | null;
}
export interface EditorWindowProps {
  document: string | null;
  editor: string | null;
}

class EditorWindow extends Component<EditorWindowProps, EditorWindowState> {
  constructor(props: EditorWindowProps) {
    super(props);
    this.state = {
      document: props.document,
      editor: props.editor
    };
  }
  render() {
    if (this.state.document) {
      return (
        <EditorLauncher
          document={this.state.document}
          editor={this.state.editor}
        ></EditorLauncher>
      );
    } else {
      return <></>;
    }
  }
}

export default EditorWindow;
