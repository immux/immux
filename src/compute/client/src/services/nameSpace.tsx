import { DeploymentUnitOutlined, AppstoreOutlined } from '@ant-design/icons';
import { ComputedNameSpace, NameSpace, Account } from '@/types/models';
import { ValuesType } from 'utility-types';
import { Moment } from 'moment';
import { Tooltip } from 'antd';
import React from 'react';

import { toMoment } from '@/utils';
import _ from 'lodash';

const accountProps = ['creator', 'updater'] as const;
const momentProps = ['createAt', 'updateAt'] as const;

type AccountProps = ValuesType<typeof accountProps>;
type MomentProps = ValuesType<typeof momentProps>;

/**
 * @param nameSpace
 * @param accounts
 */
export function toComputedNameSpace(
  nameSpace: NameSpace,
  accounts: Map<string, Account>
): ComputedNameSpace {
  const computedAccounts = _.reduce(
    accountProps,
    (acc, prop) => _.assign(acc, { [prop]: accounts.get(nameSpace[prop]) }),
    {} as { [key in AccountProps]: Account }
  );

  const computedMoments = _.reduce(
    momentProps,
    (acc, prop) => _.assign(acc, { [prop]: toMoment(nameSpace[prop]) }),
    {} as { [key in MomentProps]: Moment | null }
  );

  return _.defaults<any, any, any, any>(
    {},
    computedAccounts,
    computedMoments,
    nameSpace
  );
}

/**
 * @param nameSpace
 */
export function getNameSpaceLink(nameSpace: NameSpace | ComputedNameSpace) {
  return `/name-space/${nameSpace.id}`;
}

/**
 * @param nameSpace
 */
export function getNameSpaceIcon(nameSpace: NameSpace | ComputedNameSpace) {
  return nameSpace.root ? <DeploymentUnitOutlined /> : <AppstoreOutlined />;
}

/**
 * @param nameSpace
 * @param rootIcon
 */
export function getNameSpaceTitle(
  nameSpace: NameSpace | ComputedNameSpace,
  rootIcon = true
) {
  if (nameSpace.root) {
    return (
      <>
        {rootIcon && (
          <Tooltip title="root">{getNameSpaceIcon(nameSpace)}</Tooltip>
        )}
        {nameSpace.title || '(root)'}
      </>
    );
  }

  const name = <code>{nameSpace.name}</code>;

  return nameSpace.title ? (
    <>
      {nameSpace.title}
      <small>{name}</small>
    </>
  ) : (
    name
  );
}
