import Page from '../Page';
import React from 'react';
import styles from './PrimaryLayout.module.css';

export interface IPrimaryLayout extends React.ComponentPropsWithoutRef<'div'> {
  justify?: 'items-center' | 'items-start';
}

const PrimaryLayout: React.FC<IPrimaryLayout> = (props: IPrimaryLayout) => {
  return (
    <>
      <Page
        title={'Krondor-Rs'}
        description={'Just pointing you in the right direction...'}
        url={'krondor-rs.org'}
      >
        <div className={`min-h-screen flex flex-col ${props.justify}`}>
          <main className={styles.main}>{props.children}</main>
          <div className="m-auto" />
        </div>
      </Page>
    </>
  );
};

export default PrimaryLayout;
