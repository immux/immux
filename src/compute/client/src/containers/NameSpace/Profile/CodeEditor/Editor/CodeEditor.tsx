import { Component } from 'react';
import hotkeys from 'hotkeys-js';
import { Toaster, Position } from '@blueprintjs/core';
import { fetchSaveCode } from '@/services/api/project';

export interface CodeEditorProps {
  language: string;
  documentName: string;
}
export interface CodeEditorState {
  language: string;
  documentName: string;
  code: string;
  editor: string;
  editorData: string;
}
interface editorOptions {
  saveHotky?: boolean;
}

class CodeEditor extends Component<CodeEditorProps, CodeEditorState> {
  editorOptions?: editorOptions;
  constructor(props: CodeEditorProps) {
    super(props);
    const { store } = require('@/store');
    const { activeNode } = store.getState().project;
    const editorData = {
      code: activeNode.detail,
      editorData: {
        editor: 'Monaco',
        editorData: activeNode.key
      }
    };

    this.state = {
      language: props.language,
      editor: editorData.editorData.editor,
      documentName: props.documentName,
      code: editorData.code,
      editorData: editorData.editorData.editorData
    };
  }

  registerOptions = (opt: editorOptions) => {
    if (opt.saveHotky) {
      hotkeys('ctrl+s', (event) => {
        event.preventDefault();
        this.saveEditorDataFromState();
      });
    }
  };

  saveEditorDataFromState() {}

  updateDocument = async (
    code: string,
    editorData?: { editorData: string; editor: string }
  ) => {
    //@ts-ignore
    const pathName = window.location.pathname.match('[^/]+(?!.*/)');
    if (pathName) {
      await fetchSaveCode(pathName[0], code, this.state.editorData);
    }
  };

  saveEditorData(code: string, editorData: string) {
    if (editorData) {
      const editorDataObject = {
        editor: this.state.editor,
        editorData: editorData
      };
      console.log(editorDataObject);
    }
  }
}

export default CodeEditor;

export const Toster = Toaster.create({
  className: 'recipe-toaster',
  position: Position.BOTTOM,
  maxToasts: 5
});
