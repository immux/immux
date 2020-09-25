import React, { Component } from 'react';
import { getDocumentLanguage } from '../../utils';
import Monaco from '../../Editor/Code_Editors/Monaco_Editor/Monaco_Editor';
import { Card } from '@blueprintjs/core';

const Editors = {
  Monaco
};

interface EditorLauncherState {
  document: string | null;
  editor: string;
  editorFound: boolean;
}
interface EditorLauncherProps {
  document: string | null;
  editor?: string | undefined | null;
}

export interface CodeViewer {
  Languages: string[];
}

export interface Editor {
  name: string;
  logo: string;
  codeViewer: boolean;
  Languages: any;
}

export interface EditorsConfig {
  codeViewer: CodeViewer;
  Editors: Editor[];
}

class EditorLauncher extends Component<
  EditorLauncherProps,
  EditorLauncherState
> {
  Editors: Editor[];
  constructor(props: EditorLauncherProps) {
    super(props);
    this.Editors = require('../../config/EditorsConfig.json').Editors;
    const editor = this.getEditor(props);
    this.state = {
      document: props.document,
      editor: editor[0],
      editorFound: editor[1]
    };
  }
  getEditor(props: EditorLauncherProps): [string, boolean] {
    const document = props.document;
    if (document) {
      const documentData = '{"editor":"Monaco","editorData":""}';
      const editorName = JSON.parse(documentData).editor;
      if (editorName) {
        return [editorName, true];
      }
    }
    return ['', false];
  }
  getEditorConfig(): Editor | null {
    const editor = this.getEditorByName(this.state.editor);
    if (editor) {
      return editor;
    }
    return null;
  }
  getEditorByName(name: string): Editor | null {
    let res: Editor | null;
    res = null;
    this.Editors.forEach((editor) => {
      if (editor.name === name) {
        res = editor;
      }
    });
    return res;
  }
  getEditorLanguage = () => {
    const documentLangusage = getDocumentLanguage(this.state.document);
    const editor = this.getEditorConfig();
    if (editor) {
      const languages = editor.Languages;

      if (languages && documentLangusage) {
        const editorLanguage = languages[documentLangusage];
        if (editorLanguage) {
          return editorLanguage[0];
        }
      }
    }
    return '';
  };

  setEditorName = (name: string) => {
    if (this.state.document) {
      this.setState({ editor: name, editorFound: true });
    }
  };

  render() {
    if (this.state.editorFound && this.state.document) {
      //@ts-ignore
      return React.createElement(Editors[this.state.editor], {
        language: this.getEditorLanguage(),
        documentName: this.state.document
      });
    } else {
      return (
        <Card>
          <h3>Please select editor</h3>
        </Card>
      );
    }
  }
}

export default EditorLauncher;
