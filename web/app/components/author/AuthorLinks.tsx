import { EMAIL, LINKEDIN_URL, SOURCE_URL } from "../../links";
import styles from "./AuthorLinks.module.css";

export default function AuthorLinks() {
  return (
    <p className={styles.links}>
      <a href={SOURCE_URL} target="_blank" rel="noopener noreferrer">
        Source code
      </a>
      <a href={LINKEDIN_URL} target="_blank" rel="noopener noreferrer">
        LinkedIn
      </a>
      <a href={`mailto:${EMAIL}`}>Email</a>
    </p>
  );
}
