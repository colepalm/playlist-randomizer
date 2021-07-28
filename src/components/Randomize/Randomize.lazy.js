import React, { lazy, Suspense } from 'react';

const LazyRandomize = lazy(() => import('./Randomize'));

const Randomize = props => (
  <Suspense fallback={null}>
    <LazyRandomize {...props} />
  </Suspense>
);

export default Randomize;
