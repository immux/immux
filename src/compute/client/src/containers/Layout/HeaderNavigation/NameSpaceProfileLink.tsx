import NavigationBreadcrumb from './NavigationBreadcrumb';
import { Link } from 'react-router-dom';
import { IndexLink } from '.';
import React from 'react';

import { getNameSpaceLink, getNameSpaceTitle } from '@/services/nameSpace';
import { useStoreState } from '@/store/hooks';
import styles from './styles.module.less';
import classNames from 'classnames';

export default function NameSpaceProfileLink() {
  const nameSpace = useStoreState((state) => state.nameSpace.nameSpace);

  if (nameSpace) {
    return (
      <NavigationBreadcrumb>
        <Link
          className={classNames({ [styles.root]: nameSpace.root })}
          key={nameSpace.id}
          to={getNameSpaceLink(nameSpace)}
        >
          {getNameSpaceTitle(nameSpace)}
        </Link>
      </NavigationBreadcrumb>
    );
  }

  return <IndexLink />;
}
