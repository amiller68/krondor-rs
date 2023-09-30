import PrimaryLayout from '@components/layouts/primary/PrimaryLayout';
import { NextPageWithLayout } from './page';
import * as React from 'react';
import styles from "@styles/pages/Index.module.css";
import {config} from "@lib/config";
import router from 'next/router';

const Index: NextPageWithLayout = (_props: any) => {
  React.useEffect(() => {
    fetch("/api/rootCid")
      .then((res) => res.json())
      .then((data) => {
        // Redirect to the root CID
        const url = config.ipfs.gateway + "/ipfs/" + data.rootCid;
        router.push(url)
      }.catch((err) => {
        console.log(err);
      })
    }, []);

  return (
    <>
      <div className={styles.container}>
        <section className={styles.main}>
          <p> Redirecting you to published route...  </p>
        </section>
      </div>
    </>
  );
};

export default Index;

Index.getLayout = (page) => {
  return <PrimaryLayout>{page}</PrimaryLayout>;
};
