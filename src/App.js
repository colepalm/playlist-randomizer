import React, { Component } from "react";
import {
    BrowserRouter as Router,
    Route,
    Switch,
} from "react-router-dom";

import "./App.css";
import { Login } from './components/Login/Login'
import { Randomize } from './components/Randomize/Randomize'

class App extends Component {
  render() {
    return (
        <Router>
            <Switch>
                <Route path='/login'>
                    <Login />
                </Route>
                <Route path='/randomize'>
                    <Randomize />
                </Route>
            </Switch>
        </Router>
    );
  }
}
export default App;
