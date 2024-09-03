import { UploaderArgs, UploaderEnvs } from '../types.js';
import { logError } from './logger'
import {AxiosProxyConfig} from "axios";


export function addProxyIfNeeded(envs: UploaderEnvs, args: UploaderArgs): AxiosProxyConfig | undefined {
  if (!args.upstream) {
    return undefined
  }
  try {
    const proxyUrl = new URL(args.upstream)
    if (proxyUrl.username && proxyUrl.password) {
      return {
        host: proxyUrl.host,
        port: Number(proxyUrl.port),
        protocol: proxyUrl.protocol,
        auth: {
          username: proxyUrl.username,
          password: proxyUrl.password
        }
      }
    }
    return {
      host: proxyUrl.host,
      port: Number(proxyUrl.port),
      protocol: proxyUrl.protocol // 去掉协议末尾的 ":"
    }
  } catch (err) {
    logError(`Couldn't set upstream proxy: ${err}`)
  }

  return undefined
}
