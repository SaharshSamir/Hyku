/** Fired when the extension is first installed,
 *  when the extension is updated to a new version,
 *  and when Chrome is updated to a new version. */
import * as MessageType from "utils/constants";

let ws: WebSocket; 
chrome.runtime.onInstalled.addListener((details) => {
    console.log('[background.js] onInstalled', details);
    console.log('[background.js] onInstalled');
});

chrome.runtime.onConnect.addListener((port) => {
    console.log('[background.js] onConnect', port)
    console.log('[background.js] onInstalled');
});

chrome.runtime.onStartup.addListener(() => {
    console.log('[background.js] onStartup')
    console.log('[background.js] onInstalled');
});

/**
 *  Sent to the event page just before it is unloaded.
 *  This gives the extension opportunity to do some clean up.
 *  Note that since the page is unloading,
 *  any asynchronous operations started while handling this event
 *  are not guaranteed to complete.
 *  If more activity for the event page occurs before it gets
 *  unloaded the onSuspendCanceled event will
 *  be sent and the page won't be unloaded. */
chrome.runtime.onSuspend.addListener(() => {
    console.log('[background.js] onSuspend')
    console.log('[background.js] onSuspend');
});

chrome.runtime.onMessage.addListener((port) => {
    if(port.name === "main"){
        port.onMessage.addListener((msg:any, _:any, res:any) => {
            res(isSocketReady());
        })
    }
})

chrome.runtime.onConnect.addListener((port => {
    if(port.name === "main") {
        port.onMessage.addListener((message, something) => {
            console.log(`message: ${JSON.stringify(message)} \nsomething: ${JSON.stringify(something)}`);
            switch (message.type) {

                case MessageType.CONNECT: {
                    console.log("trying to connect...")
                    connect();
                    break;
                }
                case MessageType.SEND_MESSAGE: {
                    if(isSocketReady()){
                        ws.send("hey");
                        port.postMessage("Message sent");
                    }
                    else{
                        port.postMessage("connection not established");
                    }
                    break;
                }
                case MessageType.CHECK_CONNECTION: {
                    port.postMessage(isSocketReady());
                    break;
                }
            }
        })
    }
}))

chrome.runtime.onMessage.addListener((msg, sender, res) => {
    console.log(sender);
    switch(msg.type) {
        case MessageType.CONNECT: {
            console.log("trying to connect...")
            connect();
            break;
        }
        case MessageType.SEND_MESSAGE: {
            if(isSocketReady()){
                ws.send("hey");
                res("message sent");
            }
            else{
                res("connection not established");
            }
            break;
        }
        case MessageType.CHECK_CONNECTION: {
            res(isSocketReady());
            break;
        }
        default: break;
    }

})

function isSocketReady(): Boolean {
    if(ws && ws.readyState === 1){
        return true;
    }else {
        return false;
    }

}

function connect() {
    if(!ws) {
        ws = new WebSocket("ws://0.0.0.0:6969/ws/somethingg");
        return;
    }
    return;
}



export {}
