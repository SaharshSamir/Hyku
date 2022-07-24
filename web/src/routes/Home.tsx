import React, { useEffect, useState } from "react";
import {Payload} from "types/index";
import port from "../utils/port";
import * as MessageType from "utils/constants";

export const Home = () => {
    const [loading, setLoading] = useState(false);
    const [isConnected, setIsConnected] = useState(false);

    useEffect(() => {
        port.onMessage.addListener((msg) => {
            console.log(msg);
        })
    }, [])

    const handleConect = () => {
        console.log("huh");
        const connectMessage = {
            type: MessageType.CONNECT
        }
        port.postMessage(connectMessage);
    }
    const sendMessage = () => {
        port.postMessage({type: MessageType.SEND_MESSAGE});
    } 

    return (
        <div className="App">
            <button onClick={handleConect}>connect</button>
            <button onClick={sendMessage}>sennd message</button>
        </div>
    )
}
