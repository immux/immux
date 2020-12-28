import { HttpError } from 'routing-controllers';

import request = require('request-promise-native');
import config from '@/config';
import _ = require('lodash');

/**
 * get`AccessToken`
 * @param code
 * @param redirectUri
 */
export async function getGitAccountAccessToken(
  code: string,
  redirectUri: string
) {
  const response = await request.post({
    url: `${config.git.origin}/oauth/access_token`,
    json: true,
    formData: {
      code,
      client_id: config.git.client_id,
      client_secret: config.git.client_secret,
      redirect_uri: redirectUri,
    }
  });

  if (_.isString(response)) {
    throw new HttpError(400, response);
  }

  return {
    accessToken: _.get(response, ['access_token']) as string,
  };
}

export async function getGitAccountProfile(accessToken: string) {

  
    // * Git interface often hangs, temporarily Mock is convenient for debugging
   
    const response = await request({
      uri: `${config.git.api}/user?access_token=${accessToken}`,
      headers: {
        Authorization: `token ${accessToken}`,
        'User-Agent': 'request' // Required
      },
      json: true,
    });

    if (_.isString(response)) {
      throw new HttpError(400, response);
    }
    
    return _.mapKeys(response, (value, key) => _.camelCase(key)) as {
      id: string;
      avatarUrl: string;
      login: string;
      email: string;
    };
    
  

  // return {
  //   id: 'test',
  //   avatarUrl: 'https://avatars1.githubusercontent.com/u/10594343?s=60&v=4',
  //   login: 'immux',
  //   email: 'anchorgoogle@163.com'
  // };
}
