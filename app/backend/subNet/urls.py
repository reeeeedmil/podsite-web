from django.urls import path
from subNet import views, user_views

urlpatterns = [
        path('net-viewer/<int:pk>', views.NetViewerViewSet.as_view({'get': 'retrieve'})),
        path('net/<int:pk>', views.NetViewSet.as_view({'get': 'retrieve'})),
        path('net/', views.NetViewSet.as_view({'post': 'create'})),

        path('user/<int:pk>', user_views.UserViewSet.as_view({'get': 'retrieve'})),
        ]
