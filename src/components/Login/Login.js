import React, { Component } from 'react';

import styles from './Login.module.css';
import { BASE_URL } from "../../global/baseUrl";

export class Login extends Component {
    constructor(props) {
        super(props);
        this.state = {
            loggedIn: false,
        };
    }

    render() {
        return (
            <div className={styles.Login}>
                <div className="App">
                    <header className="App-header">
                        {!this.state.loggedIn && (
                            <a
                               href={`${BASE_URL}/login`}
                            >
                                Login to Spotify
                            </a>
                        )}
                    </header>
                </div>
            </div>
        )
    }
}
