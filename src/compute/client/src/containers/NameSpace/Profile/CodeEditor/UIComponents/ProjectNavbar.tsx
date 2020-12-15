import React, { Component } from 'react';
import { ButtonGroup, Button, Popover, Classes } from '@blueprintjs/core';
import classNames from 'classnames';

interface NavbarProps {
  document: string;
  theme: string;
  openFile: (file: string) => void;
}
interface NavbarState {
  document: string;
  theme: string;
}

export default class ProjectNavbar extends Component<NavbarProps, NavbarState> {
  constructor(props: NavbarProps) {
    super(props);
    this.state = {
      document: props.document,
      theme: props.theme
    };
  }
  render() {
    return (
      <div className={classNames('mosaic-blueprint-theme', Classes.DARK)}>
        <ButtonGroup fill minimal>
          <Popover
            minimal
            transitionDuration={10}
            hoverOpenDelay={10}
            hoverCloseDelay={10}
            interactionKind="hover"
            content={
              <ButtonGroup vertical minimal style={ButtonGroupStyle}>
                <Button
                  intent="warning"
                  text="clean editor"
                  onClick={() => {
                    this.props.openFile('');
                  }}
                ></Button>
                <Button intent="danger" text="delete"></Button>
              </ButtonGroup>
            }
          >
            <Button text="Edit"></Button>
          </Popover>

          <Popover
            minimal
            transitionDuration={10}
            hoverOpenDelay={10}
            hoverCloseDelay={10}
            interactionKind="hover"
            content={
              <ButtonGroup vertical minimal style={ButtonGroupStyle}>
                <Button text="download code"></Button>
                <Button text="download project" intent="primary"></Button>
              </ButtonGroup>
            }
          >
            <Button intent="success" text="Project"></Button>
          </Popover>
        </ButtonGroup>
        <div></div>
      </div>
    );
  }
}
const ButtonGroupStyle: React.CSSProperties = {
  width: 200
};
