import { ReactComponent as Construction } from '@/assets/icons/construction.svg';
import { ReactComponent as MenuDots } from '@/assets/icons/menu-dots.svg';
import { AntdIconProps } from '@ant-design/icons/es/components/AntdIcon';
import { ReactComponent as Logo } from '@/assets/icons/logo.svg';
import { ReactComponent as Lab } from '@/assets/icons/lab.svg';
import AntdIcon from '@ant-design/icons';
import React from 'react';

function createAntdIcon(Component: any) {
  return (props: AntdIconProps) => (
    // @ts-ignore
    <AntdIcon component={Component} {...props} />
  );
}

export const MenuDotsIcon = createAntdIcon(MenuDots);

// Logo
export const LogoIcon = createAntdIcon(Logo);

// env
export const ConstructionIcon = createAntdIcon(Construction);
export const LabIcon = createAntdIcon(Lab);
