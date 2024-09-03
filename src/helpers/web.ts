import {
  UploaderArgs,
  UploaderEnvs,
} from '../types'
import { addProxyIfNeeded } from './proxy'
import { request, setGlobalDispatcher, errors, Dispatcher } from 'undici'


export async function uploadToCanyonPOST(
  putAndResultUrlPair: any,
  canyon: any,
  envs: UploaderEnvs,
  args: UploaderArgs,
){
  return request(putAndResultUrlPair, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${canyon.reporter}`,
      'User-Agent': 'codecov-uploader/1.0.0',
    },
    body: JSON.stringify(canyon),
  })
}
