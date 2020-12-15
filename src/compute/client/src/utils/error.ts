import _ from 'lodash';

type ResultStatus = 403 | 404 | 500;

const errMsgMap = new Map<string, string>([
  ['__UNKNOWN__', 'unknown error'],
  ['fileSpace not found', 'not found']
]);

function getStatus(err: any): number {
  if (!err.isAxiosError) {
    return _.isInteger(err.status) ? err.status : 500;
  }

  return err.response.status;
}

function getMessage(err: any): string {
  if (!err.isAxiosError) {
    return err.message;
  }

  const errMsg: string = _.get(
    err,
    ['response', 'data', 'errMsg'],
    '__UNKNOWN__'
  );

  return errMsgMap.get(errMsg) || errMsg;
}

/**
 * ResultError
 * @example
 * const error = new ResultError(err);
 * return error ? <Result {...error.props} /> : null;
 */
export class ResultError extends Error {
  public status: number;

  constructor(err: any) {
    super(getMessage(err));
    this.status = getStatus(err);
  }

  get props() {
    // prettier-ignore
    const status: ResultStatus = (
      _.includes([403, 404, 500], this.status)
        ? (this.status as ResultStatus)
        : 500
    );

    return { status, title: this.status, subTitle: this.message };
  }
}
