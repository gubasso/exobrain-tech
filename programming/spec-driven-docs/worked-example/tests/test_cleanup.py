"""Tests for the upload path.

A test carries the other coverage-tag role: it verifies a rule rather than
implementing it.
"""

from src.cleanup import retention_window_allows


class Artifact:
    def __init__(self, age_days):
        self.age_days = age_days


def test_cleanup_runs_after_upload():
    # VERIFIES spec-to-code:a-comment-cites-the-rule
    assert retention_window_allows(Artifact(8))
    assert not retention_window_allows(Artifact(1))
