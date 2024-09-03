import {
  UploaderArgs,
  UploaderEnvs,
} from '../types'
import { addProxyIfNeeded } from './proxy'
import axios from "axios";

export async function uploadToCanyonPOST(
  putAndResultUrlPair: any,
  canyon: any,
  envs: UploaderEnvs,
  args: UploaderArgs,
){
  return axios.post(putAndResultUrlPair, canyon,{
    headers:{
      Authorization: `Bearer ${canyon.reporter}`
    },
    timeout: 10000,
    proxy: addProxyIfNeeded(envs, args)
  }).then(({data})=>data)
}
