import { getFunctionData } from 'reflection';
import { inspect } from 'util';

console.log(inspect(getFunctionData(getFunctionData), true));
