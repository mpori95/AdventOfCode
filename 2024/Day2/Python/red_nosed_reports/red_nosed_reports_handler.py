from typing import Optional

from ired_nosed_reports_handler import IRedNosedReportsHandler
from red_nosed_reports_context import RedNosedReportsContext

class RedNosedReportsHandler(IRedNosedReportsHandler):

    def __init__(self):
        self._next: Optional[IRedNosedReportsHandler] = None

    def Handle(self, context: RedNosedReportsContext):
        if self._next is not None:
            self._next.Handle(context)

    def set_next(self, next_handler: IRedNosedReportsHandler) -> IRedNosedReportsHandler:

        self._next = next_handler

        return self._next