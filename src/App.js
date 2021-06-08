import React, { Component } from "react";
import {
    BrowserRouter as Router,
    Switch,
    Route,
    // Link
} from "react-router-dom";

import "./App.css";
import { Login } from './components/Login/Login'

class App extends Component {
  render() {
    return (
        <Router>
            <Switch>
                <Route path='/login'>
                    <Login />
                </Route>
            </Switch>
        </Router>
    );
  }
}
export default App;
