import React, { lazy, Suspense } from 'react';

const LazyLogged-In = lazy(() => import('./Logged-In'));

const Logged-In = props => (
  <Suspense fallback={null}>
    <LazyLogged-In {...props} />
  </Suspense>
);

export default Logged-In;
