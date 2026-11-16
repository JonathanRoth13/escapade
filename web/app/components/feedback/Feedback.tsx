"use client";

import { useState, type FormEvent } from "react";
import AuthorLinks from "../author/AuthorLinks";
import Dialog from "../dialog/Dialog";
import styles from "./Feedback.module.css";

type Status = "idle" | "sending" | "sent" | "error";

export default function Feedback() {
  const [open, setOpen] = useState(false);
  const [status, setStatus] = useState<Status>("idle");
  const [error, setError] = useState("");

  const close = () => {
    setOpen(false);
    setStatus("idle");
    setError("");
  };

  const submit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = new FormData(e.currentTarget);
    setStatus("sending");
    setError("");
    try {
      const res = await fetch("/api/feedback", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(Object.fromEntries(form)),
      });
      if (!res.ok) {
        const body = await res.json().catch(() => ({}));
        throw new Error(body?.error ?? "Couldn't send your feedback.");
      }
      setStatus("sent");
    } catch (err) {
      setError((err as Error).message);
      setStatus("error");
    }
  };

  return (
    <Dialog
      label="Feedback"
      title="Feedback"
      open={open}
      onOpen={() => setOpen(true)}
      onClose={close}
    >
      {status === "sent" ? (
        <>
          <p className={styles.text}>Thanks! Your feedback was sent.</p>
          <div className={styles.actions}>
            <button type="button" className={styles.primary} onClick={close}>
              Close
            </button>
          </div>
        </>
      ) : (
        <form className={styles.form} onSubmit={submit}>
          <label className={styles.field}>
            <span>Feedback</span>
            <textarea
              name="message"
              required
              rows={5}
              maxLength={5000}
              autoFocus
              placeholder="What's confusing, broken, or could be better?"
            />
          </label>
          <label className={styles.field}>
            <span>
              Name <em>(optional)</em>
            </span>
            <input name="name" maxLength={200} autoComplete="name" />
          </label>
          <label className={styles.field}>
            <span>
              Contact info <em>(optional)</em>
            </span>
            <input name="contact" maxLength={200} />
          </label>

          {status === "error" && <p className={styles.error}>{error}</p>}

          <div className={styles.actions}>
            <button type="button" className={styles.secondary} onClick={close}>
              Cancel
            </button>
            <button
              type="submit"
              className={styles.primary}
              disabled={status === "sending"}
            >
              {status === "sending" ? "Sending…" : "Send"}
            </button>
          </div>
        </form>
      )}

      <div className={styles.links}>
        <AuthorLinks />
      </div>
    </Dialog>
  );
}
