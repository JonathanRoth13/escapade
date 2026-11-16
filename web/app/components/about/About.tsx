"use client";

import { useState } from "react";
import { SOURCE_URL } from "../../links";
import AuthorLinks from "../author/AuthorLinks";
import Dialog from "../dialog/Dialog";
import styles from "./About.module.css";

export default function About() {
  const [open, setOpen] = useState(false);

  return (
    <Dialog
      label="About"
      title="About"
      open={open}
      onOpen={() => setOpen(true)}
      onClose={() => setOpen(false)}
    >
      <div className={styles.text}>
        <p>
          Escapade is a solver that I built for Quarto. The game was first
          solved in 1998, but I&apos;ve yet to see open-source code for it, or a
          modern UI.
        </p>
        <p>
          I built it using programming techniques such as bitboards, tablebases,
          and search algorithms similar to those used in chess engines.
          There&apos;s more granular information in the{" "}
          <a
            href={`${SOURCE_URL}#readme`}
            target="_blank"
            rel="noopener noreferrer"
          >
            README
          </a>
          . The result is a Rust application that can compute the optimal move
          for any position in milliseconds, while running on a t2.micro EC2
          instance. It&apos;s by far the most technically ambitious project
          I&apos;ve completed, and I&apos;m immensely proud of it. But the best
          part of building something is getting to share it.
        </p>
        <p>
          I&apos;ve built a web application around the engine so that people can
          enjoy the game. If you&apos;ve enjoyed it or have suggestions, please
          consider filling out the feedback form.
        </p>
        <p>
          Regards,
          <br />
          Jonathan Roth
        </p>
      </div>
      <div className={styles.links}>
        <AuthorLinks />
      </div>
    </Dialog>
  );
}
