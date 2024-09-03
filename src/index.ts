import fs from 'fs'
import zlib from 'zlib'
import { version } from '../package.json'
import { info, UploadLogger } from './helpers/logger'
import { UploaderArgs } from './types'
import {uploadToCanyonPOST} from "./helpers/web";

export async function main(
  args: UploaderArgs,
): Promise<void | Record<string, unknown>> {
  // #region == Step 1: validate and sanitize inputs
  // TODO: clean and sanitize envs and args
  const envs = process.env

  // if (token === '') {
  //   info('-> No token specified or token is empty')
  // }

  let canyonJson = {}
  let coverageJson = {}

  try {
     canyonJson = JSON.parse(fs.readFileSync(process.cwd() + '/canyon.json', 'utf8'))
     coverageJson = JSON.parse(fs.readFileSync(process.cwd() + '.canyon_output/coverage-final.json', 'utf8'))
  } catch (e) {
    info('-> No canyon.json or coverage.json found')
    return
  }




  // 1.读取canyon
  const canyon:any = {
    ...canyonJson,
    coverage:coverageJson,
  }

  // 2. 上传文件

  // 3.设置代理

  const statusAndResultPair = await uploadToCanyonPOST(canyon.dsn, canyon, envs, args)


  info(JSON.stringify(statusAndResultPair))

}

export { logError, info, verbose } from './helpers/logger'
