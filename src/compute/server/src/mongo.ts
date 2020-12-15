import * as mongoose from 'mongoose';
import { debug } from '@/utils';
import config from '@/config';

mongoose.connection.on('error', err => debug.mongo(err.message));
mongoose.connection.on('connected', () => debug.mongo('Database connected'));
mongoose.connection.on('disconnected', () => debug.mongo('Database is lost'));

// noinspection JSIgnoredPromiseFromCall
mongoose.connect(config.mongo.uris, {
  ...config.mongo.options,

  useUnifiedTopology: true,
  useNewUrlParser: true,
  autoIndex: false
});

export default mongoose.connection;
