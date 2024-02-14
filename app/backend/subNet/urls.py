from django.urls import path
from subNet import views

urlpatterns = [
    path('net-viewer/<int:pk>',
         views.NetViewerViewSet.as_view({'get': 'retrieve'})),
    path('net/<int:pk>', views.NetViewSet.as_view({'get': 'retrieve'})),
    path('net/', views.NetViewSet.as_view({'post': 'create'})),
    path('prefixes/', views.PrefixPost.as_view({'post': 'create'})),
]
