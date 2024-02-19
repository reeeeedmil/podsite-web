from django.urls import path
from subNet import views

urlpatterns = [
    path('net/<string:pk>', views.NetViewSet.as_view({'get': 'retrieve'})),
    path('prefixes/', views.PrefixPost.as_view({'post': 'create'})),
    path('hosts/', views.HostPost.as_view({'post': 'create'})),
]
