export const NAME_REGEX = /^[a-z][a-z0-9]*([\-_]?[a-z0-9]+)*$/;

export const EMAIL_REGEX = /^[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$/;

export const ROOT_NAME = 'ROOT';

export enum Gender {
  Female = 0,

  Male = 1
}

// prettier-ignore
export const isDevelopment = (
  !process.env.NODE_ENV ||
  process.env.NODE_ENV === 'development'
);

export const isBeta = process.env.DEPLOY_ENV === 'beta';

export const PUBLIC_PEM_REGEX = /^-----BEGIN\s+RSA\s+PUBLIC\s+KEY-----\n[^-]+\n-----END\s+RSA\s+PUBLIC\s+KEY-----$/;

export const oreCharacters = [
         'Anderson, Kyle',
         'Bone, Jordan',
         'Bitadze, Goga',
         'Niang, Georges',
         'Okobo, Elie',
         'Mann, Terance',
         'Mika, Eric',
         'Kleber, Maxi',
         'Johnson, BJ',
         'James, LeBron',
         'Harris, Joe',
         'Hervey, Kevin',
         'Guy, Kyle',
         'George, Paul',
         'Fernando, Bruno',
         'Dozier, PJ',
         'Collins, Zach',
         'Crabbe, Allen',
         'Bolden, Jonah',
         'Booker, Devin',
         'Noah, Joakim',
         'Trier, Allonzo',
         'homas, Khyri',
         'Williams-Goss, Nigel',
         'VanVleet, Fred',
         'Vucevic, Nikola',
         'Teague, Jeff',
         'Rivers, Austin',
         'Smith, JR',
         'Rose, Derrick',
         'Paul, Chris'
       ];

/**
 * prefix file
 */
export const PRE_FILE = 'FILE';

/**
 * prefix
 */
export const PRE_DIRECTORY = 'DIRECTORY';

/**
 * extraFolders
 */
export const EXTRA_FOLDERS = ['node_modules'];

