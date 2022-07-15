import React, {useEffect, useRef} from 'react';
import { Route, Switch } from 'react-router-dom';
import { About } from "./routes/About";
import { Home } from "./routes/Home";
import port from "./utils/port";
import './App.css';

export const App = () => {
    const ws = useRef<WebSocket | null>();
    return (
        <Switch>
            <Route path="/about">
                <About/>
            </Route>
            <Route path="/">
                <Home/>
            </Route>
        </Switch>
    )
};
