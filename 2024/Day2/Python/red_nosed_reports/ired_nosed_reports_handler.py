from abc import ABC, abstractmethod

from red_nosed_reports_context import RedNosedReportsContext

class IRedNosedReportsHandler(ABC):
    @abstractmethod
    def set_next(self, next_handler: 'IRedNosedReportsHandler') -> 'IRedNosedReportsHandler':
        pass

    @abstractmethod
    def Handle(self, context: RedNosedReportsContext):
        pass
        
