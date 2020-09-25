import NameSpaceProfileLink from './NameSpaceProfileLink';
import { LogoIcon } from '@/components/icons';
import { Link } from 'react-router-dom';
import React from 'react';

import { useRouteMatch } from 'react-router-dom';

export function IndexLink() {
  return (
    <h1>
      <Link to="/">
        <LogoIcon />
        Compute Engine
      </Link>
    </h1>
  );
}

export default function HeaderNavigation() {
  const isMatchNameSpaceProfile = useRouteMatch('/name-space/:id');

  if (isMatchNameSpaceProfile) {
    return <NameSpaceProfileLink />;
  }

  return <IndexLink />;
}
