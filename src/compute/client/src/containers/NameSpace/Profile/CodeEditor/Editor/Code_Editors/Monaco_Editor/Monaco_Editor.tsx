//@ts-nocheck
import React from 'react';
import CodeEditor, { CodeEditorProps, Toster } from '../../CodeEditor';
import { Button } from '@blueprintjs/core';
import * as Monaco from 'monaco-editor';

const themeDark: Monaco.editor.IStandaloneThemeData = {
  base: 'vs-dark',
  inherit: true,
  rules: [
    { background: '#3F4B61', token: '' },
    { token: 'comment', foreground: '#ff628c' },
    { token: 'variable', foreground: '#e1efff' },
    { token: 'string', foreground: '#ffee80' }
  ],
  colors: {
    //editor
    'editor.foreground': '#CDE7F8',
    descriptionForeground: '#F29D49',
    'editor.background': '#2E3B51',
    'editor.editorCursor.foreground': '#00998C',
    'editor.lineHighlightBackground': '#4580E60f',
    'editor.selectionBackground': '#634DBF30',
    'selection.background': '#00B3A4',
    'editorWidget.background': '#1F2430',
    'editorWidget.border': '#090702',
    'editorHoverWidget.background': '#1F2430',
    'dropdown.background': '#2E3B51',
    'dropdown.border': '#9179F2',
    //scrollbarSlider
    'scrollbarSlider.background': '#00998C',
    'scrollbarSlider.hoverBackground': '#00998C',
    'scrollbarSlider.activeBackground': '#2965CC55',
    'scrollbar.shadow': '#2965CC55',
    //suggestion
    'editorSuggestWidget.background': '#1F243099',
    'editorSuggestWidget.selectedBackground': '#23525B99',
    'editorSuggestWidget.highlightForeground': '#9179F299'
  }
};
Monaco.editor.defineTheme('nnp', themeDark);

export default class MonacoEditor extends CodeEditor {
  editor: Monaco.editor.IStandaloneCodeEditor | null;
  isSaved: boolean;
  editorMount?: HTMLDivElement;
  constructor(props: CodeEditorProps) {
    super(props);
    this.editor = null;
    this.isSaved = true;
    this.editorOptions = { saveHotky: true };
    this.registerOptions(this.editorOptions);
  }
  componentWillUnmount = () => {
    const code = this.editor.getValue();
    const name = this.state.documentName;
    if (!this.isSaved) {
      Toster.show({
        message: (
          <div>
            Do you want to save the changes you made in "{name}"
            <Button
              minimal
              onClick={() => {
                this.updateDocument(code);
                Toster.clear();
              }}
              icon="tick"
              text="yes"
            />
          </div>
        ),
        intent: 'warning',
        timeout: 0
      });
    }
  };
  componentDidMount = () => {
    this.setState({ editor: 'Monaco' });
    this.onEditorMount(
      Monaco.editor.create(this.editorMount, {
        theme: 'nnp',
        language: this.state.language,
        automaticLayout: true
      })
    );
  };
  onEditorMount = (editor: Monaco.editor.IStandaloneCodeEditor) => {
    this.editor = editor;
    this.editor.setValue(this.state.code);
    this.isSaved = true;
    this.editor.addAction({
      id: 'NNP_SAVE',
      label: 'save file',
      keybindings: [2048 | 49],
      run: this.save,
      contextMenuGroupId: '9_cutcopypaste'
    });
    this.editor.onKeyDown(() => {
      this.onChange();
    });
  };

  save = () => {
    this.updateDocument(this.editor.getValue());
    Toster.show({ message: 'saved', intent: 'success' });
  };
  onChange = () => {
    this.isSaved = false;
  };
  render() {
    return (
      <div
        style={{ width: '100%', height: '100%' }}
        ref={(div) => {
          this.editorMount = div;
        }}
      />
    );
  }
}
