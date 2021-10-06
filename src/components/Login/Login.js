import React, { Component } from 'react';
// import PropTypes from 'prop-types';

import styles from './Login.module.css';

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
                            <a className="btn btn--loginApp-link"
                               href={`https://serene-cove-58239.herokuapp.com/login`}
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
