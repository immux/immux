import * as fs from 'fs';
import * as path from 'path';
import { FileNodeInfo } from '@/types';
import { PRE_FILE, PRE_DIRECTORY, EXTRA_FOLDERS } from '@/constants';

/**
 * path filter
 * @desc exclude basename path(for example .svn, _html,  _a.psd)
 */
export function defaultFilter(uri: string) {
  let start = path.basename(uri).charAt(0);
  if (start === '.' || start === '_') {
    start = null;
    return false;
  }
  return true;
}

/**
 * traversal tree
 * */
export function walkTree(uri: string, filter: Function) {
  const node: FileNodeInfo = {
    title: '',
    key: '',
    children: [],
    pNode: ''
  };

  if (filter(uri)) {
    let stat = fs.lstatSync(uri);

    if (stat.isFile()) {
      uri = path.resolve(uri);

      node.title = path.basename(uri);
      node.key = `${PRE_FILE}${uri}`;
      node.extname = path.extname(uri);
      node.detail = fs.readFileSync(uri, 'utf-8');
      node.isLeaf = true;
    }

    if (stat.isDirectory()) {
      node.title = path.basename(uri);
      node.key = `${PRE_DIRECTORY}${uri}`;

      fs.readdirSync(uri).forEach((part) => {
        if (!EXTRA_FOLDERS.includes(part)) {
          let childTree = walkTree(path.join(uri, part), filter);

          if (childTree.title) {
            childTree.pNode = node.title;
            node.children.push(childTree);
          }

          childTree = null;
        }
      });
    }

    stat = null;
  }

  return node;
}

/**
 * start recursive traversal
 * */
export function recursiveTraversalWalk(rootDir: string, filter?: Function) {
  return walkTree(rootDir, filter || defaultFilter);
}
