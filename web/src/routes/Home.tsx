import React, { useEffect, useState } from "react";
import {Payload} from "types/index";
import port from "../utils/port";
import * as MessageType from "utils/constants";

export const Home = () => {
    const [loading, setLoading] = useState(false);
    const [isConnected, setIsConnected] = useState(false);

    //comment stuff
    const payload: Payload = {
        from: "something",
        to: "someting else",
        crossUser: false,
        msg: "ur mum says hi"
    }
    const handleConect = () => {
        const connectMessage = {
            type: MessageType.CONNECT
        }
        port.postMessage(connectMessage);
    }
    port.postMessage({type: "SEND_MESSAGE"})

    return (
        <div className="App">
            <button onClick={handleConect}>connect</button>
            <button>sennd message</button>
        </div>
    )
}
