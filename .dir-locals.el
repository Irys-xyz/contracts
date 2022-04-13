((nil . ()) ;; no settings for all modes
 (rustic-mode ;; for rustic mode
  ;; ensure the buffer workspace is always set to the project root to work around
  ;; brotzeit/rustic#179 and brotzeit/rustic#236
  . ((eval . (advice-add 'rustic-buffer-workspace :override (lambda (&optional ignored) (format "%srust/" (projectile-project-root))))))))
