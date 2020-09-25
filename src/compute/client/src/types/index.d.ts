import { presetColors } from '@/constants';
import { ValuesType } from 'utility-types';

export interface Dictionary<T> {
  [index: string]: T;
}

export type PresetColors = ValuesType<typeof presetColors>;
