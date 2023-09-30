import { NextApiRequest, NextApiResponse } from 'next';
import RootCid from "@lib/eth";
import Config from '../../../krondor.json';

const RPC_URL = Config.eth.rpc_url + '/' + process.env.NEXT_PRIVATE_RPC_API_KEY;
const CONTRACT_ADDRESS = Config.eth.contract_address;
const CHAIN_ID = Config.eth.chain_id;

export default async function handler(
    _req: NextApiRequest,
    res: NextApiResponse
    ) 
{
    const rootCid = new RootCid(RPC_URL, CONTRACT_ADDRESS, CHAIN_ID);
    const cid = await rootCid.get();
    res.status(200).json({ cid });
}