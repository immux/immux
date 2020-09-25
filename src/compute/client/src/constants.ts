export const STORAGE_ACCESS_TOKEN = 'accessToken';

export const STORAGE_HISTORIES = 'histories';

export const betaLink = '';
export const releaseLink = '';

export const deployEnv = process.env.REACT_APP_DEPLOY_ENV || 'development';

export const deployEnvLabel = (() => {
  switch (deployEnv) {
    case 'development': {
      return 'development';
    }

    case 'beta': {
      return 'beta';
    }

    case 'release': {
      return 'release';
    }

    default: {
      return 'default';
    }
  }
})();

export const primaryColor = '#ff7c0a';

export const presetColors = [
  'pink',
  'magenta',
  'red',
  'volcano',
  'orange',
  'yellow',
  'gold',
  'cyan',
  'lime',
  'green',
  'blue',
  'geekblue',
  'purple'
] as const;

export enum Gender {
  Female = 0,
  Male = 1
}

export const ROOT_NAME = 'ROOT';

export const PROJECT_MOCK_URL = 'http://localhost:3000/';
