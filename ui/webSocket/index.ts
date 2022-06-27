import {
    AppWebsocket,
    CallZomeRequest,
} from "@holochain/client";

const H_APP_ID = 'holo-go'; 
const WS_URL = 'ws://localhost:8888';

let  appInfo, cell_id, appClient;

interface ZomeInput {
    data: String;
}

interface ZomeOutput{
    data: String;
}

export const initConnection = async () =>{
    console.log(process.env.REACT_APP_HC_PORT);
    const portReact = process.env.REACT_APP_HC_PORT;
    if (!portReact){
        throw new Error(
            "No port React App"
        )
    }else{
        AppWebsocket.connect(portReact).then(
            async (appClient) => {
                appInfo = await appClient.appInfo({installed_app_id: H_APP_ID });
                if (!appInfo.cell_data[0]){
                    throw new Error('No app info found');
                }
                cell_id = appInfo.cell_data[0].cell_id;  
            }
        )

    }
}

export const call = async ({ fnName, zomeName, data }) => {
    if (!AppWebsocket || !appInfo || !cell_id){
        throw new Error(
            "Cant establishement connection with Holo"
        );
    }else{
        //const payload: ZomeInput = {value: data}
        const apiRequest: CallZomeRequest = 
        {
            cap_secret: null,
            cell_id,
            zome_name: zomeName,
            fn_name: fnName,
            provenance: cell_id[1],
            payload: {
                value: data
            }
        };
        
        try {
            const outPut: ZomeOutput = await appClient.CallZome(apiRequest);
            console.log('Result of the call ', outPut)
        } catch(error) {
            console.log('Error ', error)
        }finally{
            appClient.client.CallZomeRequest();
        }
    }    
}



